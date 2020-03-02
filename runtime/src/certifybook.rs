/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{
	decl_module, decl_storage, decl_event, dispatch::DispatchResult, StorageValue, StorageMap,
	weights::{SimpleDispatchInfo, DispatchInfo, DispatchClass, ClassifyDispatch, WeighData, Weight, PaysFee}
};
use system::ensure_signed;

/// The module's configuration trait.
pub trait Trait: system::Trait {
	// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.

decl_storage! {
	trait Store for Module<T: Trait> as CertifybookModule {
		// Just a dummy storage item.

		// //Certificates get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;
        // KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;

        AllCertificatesArray get(certificate_by_index): map u64 => T::Hash;
        AllCertificatesCount get(all_certificates_count): u64;
        //AllCertificatesIndex: map T::Hash => u64;

		OrgCertificatesArray get(certificate_of_org_by_index): map (T::AccountId, u64) => T::Hash;
		// The count of certificates issued by organizations
		OrgCertificatesCount get(certificates_count_of_org): map T::AccountId => u64;
		//OrgCertificatesIndex: map T::Hash => u64;
	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event() = default;

		// Just a dummy entry point.
		// function that can be called by the external world as an extrinsics call
		// takes a parameter of the type `AccountId`, stores it and emits an event
		#[weight = SimpleDispatchInfo::FixedNormal(0)]
		pub fn new_certificate(origin, certificate: T::Hash) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let all_certificates_count = Self::all_certificates_count();
			let new_all_certificates_count = all_certificates_count.checked_add(1).ok_or("Overflow add a new certificate")?;
			AllCertificatesCount::put(new_all_certificates_count);
			<AllCertificatesArray<T>>::insert(all_certificates_count, certificate);

			let certificates_count_of_org = Self::certificates_count_of_org(&who);
			let new_certificates_count_of_org = certificates_count_of_org.checked_add(1).ok_or("Overflow add a new certificate")?;
			<OrgCertificatesCount<T>>::insert(who.clone(), new_certificates_count_of_org);
			<OrgCertificatesArray<T>>::insert((who.clone(), certificates_count_of_org), certificate);


			// here we are raising the Something event
			//Self::deposit_event(RawEvent::SomethingStored(something, who));
			Self::deposit_event(RawEvent::CertificateStored(certificate, who));
			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> 
	where 
		AccountId = <T as system::Trait>::AccountId,
		Hash = <T as system::Trait>::Hash {
			// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		CertificateStored(Hash, AccountId),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use sp_core::H256;
	use frame_support::{impl_outer_origin, assert_ok, parameter_types, weights::Weight};
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
		type ModuleToIndex = ();
	}
	impl Trait for Test {
		type Event = ();
	}
	type CertifybookModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> sp_io::TestExternalities {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn it_works_for_default_value() {
		new_test_ext().execute_with(|| {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(CertifybookModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(CertifybookModule::something(), Some(42));
		});
	}
}
