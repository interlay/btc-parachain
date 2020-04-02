use crate::mock::{run_test, ExchangeRateOracle, Origin, System, Test, TestEvent};
use crate::Error;

use frame_support::{assert_err, assert_ok};
use mocktopus::mocking::*;

type Event = crate::Event<Test>;

// use macro to avoid messing up stack trace
macro_rules! assert_emitted {
    ($event:expr) => {
        let test_event = TestEvent::test_events($event);
        assert!(System::events().iter().any(|a| a.event == test_event));
    };
}

macro_rules! assert_not_emitted {
    ($event:expr) => {
        let test_event = TestEvent::test_events($event);
        assert!(!System::events().iter().any(|a| a.event == test_event));
    };
}

#[test]
fn set_exchange_rate_success() {
    run_test(|| {
        ExchangeRateOracle::get_authorized_oracle.mock_safe(|| MockResult::Return(3));
        let result = ExchangeRateOracle::set_exchange_rate(Origin::signed(3), 100);
        assert_ok!(result);

        let exchange_rate = ExchangeRateOracle::get_exchange_rate().unwrap();
        assert_eq!(exchange_rate, 100);

        assert_emitted!(Event::SetExchangeRate(3, 100));
    });
}

#[test]
fn set_exchange_rate_max_delay_passed() {
    run_test(|| {
        let mut first_call_to_recover = false;
        ExchangeRateOracle::get_authorized_oracle.mock_safe(|| MockResult::Return(3));
        ExchangeRateOracle::is_max_delay_passed.mock_safe(|| MockResult::Return(Ok(true)));
        // XXX: hacky way to ensure that `recover_from_oracle_offline` was
        // indeed called. mocktopus does not seem to have a `assert_called`
        // kind of feature yet
        ExchangeRateOracle::recover_from_oracle_offline.mock_safe(move || {
            MockResult::Return(if first_call_to_recover {
                Err(Error::RuntimeError)
            } else {
                first_call_to_recover = true;
                Ok(())
            })
        });
        let first_res = ExchangeRateOracle::set_exchange_rate(Origin::signed(3), 100);
        assert_ok!(first_res);

        let second_res = ExchangeRateOracle::set_exchange_rate(Origin::signed(3), 100);
        assert_err!(second_res, Error::RuntimeError);
    });
}

#[test]
fn set_exchange_rate_wrong_oracle() {
    run_test(|| {
        ExchangeRateOracle::get_authorized_oracle.mock_safe(|| MockResult::Return(4));
        assert_ok!(ExchangeRateOracle::set_exchange_rate(Origin::signed(4), 20));

        let result = ExchangeRateOracle::set_exchange_rate(Origin::signed(3), 100);
        assert_err!(result, Error::InvalidOracleSource);

        let exchange_rate = ExchangeRateOracle::get_exchange_rate().unwrap();
        assert_eq!(exchange_rate, 20);

        assert_not_emitted!(Event::SetExchangeRate(3, 100));
        assert_not_emitted!(Event::SetExchangeRate(4, 100));
    });
}

#[test]
fn get_exchange_rate_after_delay() {
    run_test(|| {
        ExchangeRateOracle::is_max_delay_passed.mock_safe(|| MockResult::Return(Ok(true)));
        let result = ExchangeRateOracle::get_exchange_rate();
        assert_err!(result, Error::MissingExchangeRate);
    });
}

#[test]
fn is_max_delay_passed() {
    run_test(|| {
        let now = 1585776145;

        ExchangeRateOracle::seconds_since_epoch.mock_safe(move || MockResult::Return(Ok(now)));
        ExchangeRateOracle::get_last_exchange_rate_time
            .mock_safe(move || MockResult::Return(now - 3600));

        // max delay is 30 minutes but 1 hour passed
        ExchangeRateOracle::get_max_delay.mock_safe(|| MockResult::Return(1800));
        assert!(ExchangeRateOracle::is_max_delay_passed().unwrap());

        // max delay is 2 hours and 1 hour passed
        ExchangeRateOracle::get_max_delay.mock_safe(|| MockResult::Return(7200));
        assert!(!ExchangeRateOracle::is_max_delay_passed().unwrap());
    });
}

#[test]
fn seconds_since_epoch() {
    run_test(|| {
        let now = 1585776145;
        let ten_years = 3600 * 24 * 365;
        let timestamp = ExchangeRateOracle::seconds_since_epoch().unwrap();
        // check that the value of timestamp looks reasonable
        // this test will start failing in 2030 or so
        assert!(now - ten_years < timestamp);
        assert!(timestamp < now + ten_years);
    });
}
