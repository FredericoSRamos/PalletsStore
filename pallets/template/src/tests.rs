#![cfg(test)]

use super::*;
use crate::mock::*;
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_adds_a_product() {
    new_test_ext().execute_with(|| {
        let product_name = b"Test Product".to_vec();
        let stock = 100;
        let price = 50;
        let amount_to_restock = 20;
        let restock_date = Date::new(1, 1, 2023).unwrap();
        let category = Category::Electronic;

        assert_ok!(Pallet::<Test>::add_product(
            RuntimeOrigin::signed(1),
            product_name.clone(),
            stock,
            price,
            amount_to_restock,
            restock_date.clone(),
            category.clone()
        ));

        let product = Products::<Test>::get(0).unwrap();
        assert_eq!(product.name, product_name);
        assert_eq!(product.stock, stock);
        assert_eq!(product.price, price);
        assert_eq!(product.amount_to_restock, amount_to_restock);
        assert_eq!(product.restock_date, restock_date);
        assert_eq!(product.category, category);
    });
}

#[test]
fn it_gets_a_product() {
    new_test_ext().execute_with(|| {
        let product_name = b"Test Product".to_vec();
        let stock = 100;
        let price = 50;
        let amount_to_restock = 20;
        let restock_date = Date::new(1, 1, 2023).unwrap();
        let category = Category::Electronic;

        assert_ok!(Pallet::<Test>::add_product(
            RuntimeOrigin::signed(1),
            product_name.clone(),
            stock,
            price,
            amount_to_restock,
            restock_date,
            category
        ));

        assert_ok!(Pallet::<Test>::get_product(RuntimeOrigin::signed(1), 0));
    });
}

#[test]
fn it_fails_to_get_a_nonexistent_product() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Pallet::<Test>::get_product(RuntimeOrigin::signed(1), 999),
            Error::<Test>::ProductNotFound
        );
    });
}

#[test]
fn it_lists_products_to_restock() {
    new_test_ext().execute_with(|| {
        let product_name = b"Test Product".to_vec();
        let stock = 5;
        let price = 50;
        let amount_to_restock = 10;
        let restock_date = Date::new(1, 1, 2023).unwrap();
        let category = Category::Electronic;

        assert_ok!(Pallet::<Test>::add_product(
            RuntimeOrigin::signed(1),
            product_name.clone(),
            stock,
            price,
            amount_to_restock,
            restock_date,
            category
        ));

        assert_ok!(Pallet::<Test>::list_products_to_restock(RuntimeOrigin::signed(1)));
    });
}

#[test]
fn it_updates_a_product() {
    new_test_ext().execute_with(|| {
        let product_name = b"Test Product".to_vec();
        let stock = 100;
        let price = 50;
        let amount_to_restock = 20;
        let restock_date = Date::new(1, 1, 2023).unwrap();
        let category = Category::Electronic;

        assert_ok!(Pallet::<Test>::add_product(
            RuntimeOrigin::signed(1),
            product_name.clone(),
            stock,
            price,
            amount_to_restock,
            restock_date,
            category
        ));

        let new_name = b"Updated Product".to_vec();
        assert_ok!(Pallet::<Test>::update_product(
            RuntimeOrigin::signed(1),
            0,
            Some(new_name.clone()),
            None,
            None,
            None,
            None,
			None
        ));

        let product = Products::<Test>::get(0).unwrap();
        assert_eq!(product.name, new_name);
    });
}

#[test]
fn it_fails_to_update_a_nonexistent_product() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Pallet::<Test>::update_product(RuntimeOrigin::signed(1), 999, None, None, None, None, None, None),
            Error::<Test>::ProductNotFound
        );
    });
}

#[test]
fn it_removes_a_product() {
    new_test_ext().execute_with(|| {
        let product_name = b"Test Product".to_vec();
        let stock = 100;
        let price = 50;
        let amount_to_restock = 20;
        let restock_date = Date::new(1, 1, 2023).unwrap();
        let category = Category::Electronic;

        assert_ok!(Pallet::<Test>::add_product(
            RuntimeOrigin::signed(1),
            product_name.clone(),
            stock,
            price,
            amount_to_restock,
            restock_date,
            category
        ));

        // Remove the product
        assert_ok!(Pallet::<Test>::remove_product(RuntimeOrigin::signed(1), 0));
        assert_noop!(
            Pallet::<Test>::get_product(RuntimeOrigin::signed(1), 0),
            Error::<Test>::ProductNotFound
        );
    });
}

#[test]
fn it_registers_a_sale() {
    new_test_ext().execute_with(|| {
        let product_name = b"Test Product".to_vec();
        let stock = 100;
        let price = 50;
        let amount_to_restock = 20;
        let restock_date = Date::new(1, 1, 2023).unwrap();
        let category = Category::Electronic;

        assert_ok!(Pallet::<Test>::add_product(
            RuntimeOrigin::signed(1),
            product_name.clone(),
            stock,
            price,
            amount_to_restock,
            restock_date,
            category
        ));

        let seller = b"Test Seller".to_vec();
        let products = vec![ItemSale { product_id: 0, amount: 2 }];
        let payment_method = PaymentMethod::Credit;

        assert_ok!(Pallet::<Test>::register_sale(
            RuntimeOrigin::signed(1),
            seller.clone(),
            products.clone(),
            payment_method
        ));

        let sale = Sales::<Test>::get(0).unwrap();
        assert_eq!(sale.seller, seller);
        assert_eq!(sale.products, vec![0]);
        assert_eq!(sale.value, 100); // 2 * 50
    });
}

#[test]
fn it_fails_to_register_a_sale_with_insufficient_stock() {
    new_test_ext().execute_with(|| {
        let product_name = b"Test Product".to_vec();
        let stock = 1;
        let price = 50;
        let amount_to_restock = 20;
        let restock_date = Date::new(1, 1, 2023).unwrap();
        let category = Category::Electronic;

        assert_ok!(Pallet::<Test>::add_product(
            RuntimeOrigin::signed(1),
            product_name.clone(),
            stock,
            price,
            amount_to_restock,
            restock_date,
            category
        ));

        let seller = b"Test Seller".to_vec();
        let products = vec![ItemSale { product_id: 0, amount: 2 }];
        let payment_method = PaymentMethod::Credit;

        assert_noop!(
            Pallet::<Test>::register_sale(RuntimeOrigin::signed(1), seller, products, payment_method),
            Error::<Test>::InsufficientStock
        );
    });
}

#[test]
fn it_fails_to_register_a_sale_with_nonexistent_product() {
    new_test_ext().execute_with(|| {
        let seller = b"Test Seller".to_vec();
        let products = vec![ItemSale { product_id: 999, amount: 1 }];
        let payment_method = PaymentMethod::Credit;

        assert_noop!(
            Pallet::<Test>::register_sale(RuntimeOrigin::signed(1), seller, products, payment_method),
            Error::<Test>::ProductNotFound
        );
    });
}

#[test]
fn it_lists_all_sales() {
    new_test_ext().execute_with(|| {
        let product_name = b"Test Product".to_vec();
        let stock = 100;
        let price = 50;
        let amount_to_restock = 20;
        let restock_date = Date::new(1, 1, 2023).unwrap();
        let category = Category::Electronic;

        assert_ok!(Pallet::<Test>::add_product(
            RuntimeOrigin::signed(1),
            product_name.clone(),
            stock,
            price,
            amount_to_restock,
            restock_date,
            category
        ));

        let seller = b"Test Seller".to_vec();
        let products = vec![ItemSale { product_id: 0, amount: 2 }];
        let payment_method = PaymentMethod::Credit;

        assert_ok!(Pallet::<Test>::register_sale(
            RuntimeOrigin::signed(1),
            seller.clone(),
            products.clone(),
            payment_method
        ));

        assert_ok!(Pallet::<Test>::list_all_sales(RuntimeOrigin::signed(1)));
    });
}