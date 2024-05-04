use validator::ValidateEmail;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if ValidateEmail::validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid email address.", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use std::u32;

    use claims::assert_err;
    use fake::{faker::internet::en::SafeEmail, Fake};
    use proptest::prelude::*;

    use crate::domain::SubscriberEmail;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    // impl quickcheck::Arbitrary for ValidEmailFixture {
    //     fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
    //         let email = SafeEmail().fake_with_rng(g);
    //         Self(email)
    //     }
    // }
    fn email() -> impl Strategy<Value = ValidEmailFixture> {
        any::<u32>().prop_map(|_| ValidEmailFixture(SafeEmail().fake()))
    }
    //
    // #[quickcheck_macros::quickcheck]
    // fn valid_email_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
    //     dbg!(&valid_email.0);
    //     SubscriberEmail::parse(valid_email.0).is_ok()
    // }
    proptest! {
         #[test]
        fn valid_emails_are_parsed_succesfully(valid_email in email()) {
            claims::assert_ok!(SubscriberEmail::parse(dbg!(valid_email.0)));
    }

    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
}
