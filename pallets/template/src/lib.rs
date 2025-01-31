#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {

	use super::*;
	use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

	#[derive(Clone, Encode, Decode, Debug, TypeInfo, PartialEq, Eq, MaxEncodedLen)]
    pub enum Category {
        Electronic,
        Food,
        Clothing,
        Misc
    }

    #[derive(Clone, Encode, Decode, TypeInfo, PartialEq, Debug, Eq, MaxEncodedLen)]
    pub enum PaymentMethod {
        Credit,
        Debit,
        Pix,
        Money
    }

    #[derive(Clone, Encode, Decode, Debug, TypeInfo, Default, PartialEq, MaxEncodedLen)]
    pub struct Date {
        day: u8,
        month: u8,
        year: u64
    }

    impl Date {
        pub fn new(day: u8, month: u8, year: u64) -> Result<Self, &'static str> {
            if !(1..=31).contains(&day) || !(1..=12).contains(&month) || year < 1000 {
                return Err("Invalid date");
            }
            Ok(Self { day, month, year })
        }
    }

    #[derive(Clone, Encode, Decode, Debug, PartialEq, TypeInfo)]
    pub struct ItemSale {
        product_id: u64,
        amount: u64
    }

    #[derive(Clone, Debug, Encode, Decode, PartialEq, TypeInfo)]
    pub struct Product {
        name: Vec<u8>,
        id: u64,
        stock: u64,
        price: u64,
        amount_to_restock: u64,
        restock_date: Date,
        category: Category
    }

	impl MaxEncodedLen for Product {
		fn max_encoded_len() -> usize {
			let max_name_length = 256;
			let max_date_length = Date::max_encoded_len();
			let max_category_length = Category::max_encoded_len();
	
			max_name_length + max_date_length + max_category_length + 32
		}
	}

    #[derive(Clone, Debug, Encode, Decode, PartialEq, TypeInfo)]
    pub struct Sale {
        seller: Vec<u8>,
        code: u64,
        products: Vec<u64>,
        value: u64,
        date: Date,
        payment_method: PaymentMethod
    }

	impl MaxEncodedLen for Sale {
		fn max_encoded_len() -> usize {
			let seller_length = 256;
			let date_length = Date::max_encoded_len();
			let payment_method_length = PaymentMethod::max_encoded_len();
		
			seller_length + date_length + payment_method_length + 96
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
    #[pallet::getter(fn products)]
    pub type Products<T> = StorageMap<_, Blake2_128Concat, u64, Product, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn sales)]
    pub type Sales<T> = StorageMap<_, Blake2_128Concat, u64, Sale, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_product_id)]
    pub type NextProductId<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_sale_code)]
    pub type NextSaleCode<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProductAdded(u64),
		ProductGotten(Product),
		ProductsListed(Vec<Product>),
		ProductUpdated(u64),
        ProductRemoved(u64),
        SaleRegistered(u64),
		SaleGotten(Sale),
		SalesListed(Vec<Sale>),
        SaleUpdated(u64),
        SaleRemoved(u64)
	}

	#[pallet::error]
	pub enum Error<T> {
		ProductNotFound,
        SaleNotFound,
        InsufficientStock,
        InvalidDate,
        Overflow
	}

	#[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
		#[pallet::weight(10_000)]
        pub fn add_product( origin: OriginFor<T>, name: Vec<u8>, stock: u64, price: u64, amount_to_restock: u64, restock_date: Date, category: Category) -> DispatchResult {
            let _who = ensure_signed(origin)?;

			let restock_date = Date::new(restock_date.day, restock_date.month, restock_date.year).map_err(|_| Error::<T>::InvalidDate)?;

            let product_id = Self::next_product_id();

            let product = Product {
                name,
                id: product_id,
                stock,
                price,
                amount_to_restock,
                restock_date,
                category
            };

            Products::<T>::insert(product_id, product);
            NextProductId::<T>::put(product_id + 1);
            Self::deposit_event(Event::ProductAdded(product_id));

            Ok(())
        }

		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn get_product(origin: OriginFor<T>, id: u64) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			if let Some(product) = Products::<T>::get(id) {
				Self::deposit_event(Event::ProductGotten(product));
				Ok(())
			} else {
				Err(Error::<T>::ProductNotFound.into())
			}
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn list_all_products(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let products: Vec<Product> = Products::<T>::iter().map(|(_, product)| product).collect();
			Self::deposit_event(Event::ProductsListed(products));

			Ok(())
		}

        #[pallet::call_index(3)]
		#[pallet::weight(10_000)]
        pub fn update_product(origin: OriginFor<T>, id: u64, name: Option<Vec<u8>>, stock: Option<u64>, price: Option<u64>, amount_to_restock: Option<u64>, restock_date: Option<Date>, category: Option<Category>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let mut product = Products::<T>::get(id).ok_or(Error::<T>::ProductNotFound)?;

            if let Some(new_name) = name {
                product.name = new_name;
            }

            if let Some(new_stock) = stock {
                product.stock = new_stock;
            }

            if let Some(new_price) = price {
                product.price = new_price;
            }

            if let Some(new_amount_to_restock) = amount_to_restock {
                product.amount_to_restock = new_amount_to_restock;
            }

            if let Some(new_restock_date) = restock_date {
                product.restock_date = new_restock_date;
            }

            if let Some(new_category) = category {
                product.category = new_category;
            }

            Products::<T>::insert(id, product);
            Self::deposit_event(Event::ProductUpdated(id));

            Ok(())
        }

        #[pallet::call_index(4)]
		#[pallet::weight(10_000)]
        pub fn remove_product(origin: OriginFor<T>, id: u64) -> DispatchResult {
            let _who = ensure_signed(origin)?;

			ensure!(Products::<T>::contains_key(id), Error::<T>::ProductNotFound);

            Products::<T>::remove(id);
            Self::deposit_event(Event::ProductRemoved(id));

            Ok(())
        }

        #[pallet::call_index(5)]
		#[pallet::weight(10_000)]
        pub fn register_sale(origin: OriginFor<T>, seller: Vec<u8>, products: Vec<ItemSale>, payment_method: PaymentMethod) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let mut total_value: u64 = 0;
            let mut sale_products: Vec<u64> = Vec::new();

            for item in products {
                let mut product = Products::<T>::get(item.product_id).ok_or(Error::<T>::ProductNotFound)?;
                product.stock = product.stock.checked_sub(item.amount).ok_or(Error::<T>::InsufficientStock)?;
                Products::<T>::insert(item.product_id, &product);

                if !sale_products.contains(&item.product_id) {
                    sale_products.push(item.product_id);
                }

                let partial_value = product.price.checked_mul(item.amount).ok_or(Error::<T>::Overflow)?;
                total_value = total_value.checked_add(partial_value).ok_or(Error::<T>::Overflow)?;
            }

            let sale_code = Self::next_sale_code();
            let sale = Sale {
                seller,
                code: sale_code,
                products: sale_products,
                value: total_value,
                date: Date::new(3, 2, 2025).unwrap(),
                payment_method
            };

            Sales::<T>::insert(sale_code, sale);
            NextSaleCode::<T>::put(sale_code + 1);
            Self::deposit_event(Event::SaleRegistered(sale_code));

            Ok(())
        }

		#[pallet::call_index(6)]
		#[pallet::weight(10_000)]
		pub fn get_sale(origin: OriginFor<T>, code: u64) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			if let Some(sale) = Sales::<T>::get(code) {
				Self::deposit_event(Event::SaleGotten(sale));
				Ok(())
			} else {
				Err(Error::<T>::SaleNotFound.into())
			}
		}

		#[pallet::call_index(7)]
		#[pallet::weight(10_000)]
		pub fn list_all_sales(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let sales: Vec<Sale> = Sales::<T>::iter().map(|(_, sale)| sale).collect();
			Self::deposit_event(Event::SalesListed(sales));

			Ok(())
		}

        #[pallet::call_index(8)]
		#[pallet::weight(10_000)]
        pub fn update_sale(origin: OriginFor<T>, code: u64, seller: Option<Vec<u8>>, date: Option<Date>, payment_method: Option<PaymentMethod>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let mut sale = Sales::<T>::get(code).ok_or(Error::<T>::SaleNotFound)?;

            if let Some(new_seller) = seller {
                sale.seller = new_seller;
            }

            if let Some(new_date) = date {
				let new_date = Date::new(new_date.day, new_date.month, new_date.year).map_err(|_| Error::<T>::InvalidDate)?;
                sale.date = new_date;
            }

            if let Some(new_payment_method) = payment_method {
                sale.payment_method = new_payment_method;
            }

            Sales::<T>::insert(code, sale);
            Self::deposit_event(Event::SaleUpdated(code));

            Ok(())
        }

        #[pallet::call_index(9)]
		#[pallet::weight(10_000)]
        pub fn remove_sale(origin: OriginFor<T>, code: u64) -> DispatchResult {
            let _who = ensure_signed(origin)?;

			ensure!(Sales::<T>::contains_key(code), Error::<T>::SaleNotFound);

            Sales::<T>::remove(code);
            Self::deposit_event(Event::SaleRemoved(code));

            Ok(())
        }
    }
}