use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;


#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim),
		(1, frame_system::Pallet::<Test>::block_number()))
	});
}

#[test]
fn create_claim_failed_when_claim_already_exists() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyClaimed
        );

    });
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
       
    });
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::NoSuchProof
        );
    })
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(),3));
        assert_eq!(Proofs::<Test>::get(&claim),
            (3, frame_system::Pallet::<Test>::block_number()))
    });
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1),  claim.clone(), 1),
            Error::<Test>::NoSuchProof
        );
    })
}

#[test]
fn transfer_claim_failed_when_sender_is_not_owner() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(3), claim.clone(), 2),
            Error::<Test>::NotProofOwner
        );
    })
}
///if the long of proof is longer than 10, we will pass this test
#[test]
fn create_claim_failed_when_proof_is_long() {
	
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0,1,2,3,4,5,6,7,8,9];
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim),
			Error::<Test>::ProofTooLong
		);
	});
}

