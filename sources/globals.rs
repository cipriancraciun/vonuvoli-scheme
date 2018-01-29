

use super::counters::*;
use super::runtime::*;




pub(crate) mod exports {
	pub(crate) use super::context_handles_next;
	pub(crate) use super::bindings_handles_next;
	pub(crate) use super::records_handles_next;
	pub(crate) use super::lambdas_handles_next;
	pub(crate) use super::ports_handles_next;
	pub(crate) use super::processes_handles_next;
}




static mut CONTEXT_HANDLES : PermutationCounter = PermutationCounter {
		index : 0x514765cd,
		offset : 0x4d564bb6,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn context_handles_next () -> (Handle) {
	unsafe {
		Handle::new (CONTEXT_HANDLES.next ())
	}
}




static mut BINDINGS_HANDLES : PermutationCounter = PermutationCounter {
		index : 0xc8b5516d,
		offset : 0x767a8d5c,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn bindings_handles_next () -> (Handle) {
	unsafe {
		Handle::new (BINDINGS_HANDLES.next ())
	}
}




static mut RECORDS_HANDLES : PermutationCounter = PermutationCounter {
		index : 0x8251b601,
		offset : 0xac4eff38,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn records_handles_next () -> (Handle) {
	unsafe {
		Handle::new (RECORDS_HANDLES.next ())
	}
}




static mut LAMBDAS_HANDLES : PermutationCounter = PermutationCounter {
		index : 0x33faad68,
		offset : 0xe28c918f,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn lambdas_handles_next () -> (Handle) {
	unsafe {
		Handle::new (LAMBDAS_HANDLES.next ())
	}
}




static mut PORTS_HANDLES : PermutationCounter = PermutationCounter {
		index : 0xa7e6ecf7,
		offset : 0x1e5c47b7,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn ports_handles_next () -> (Handle) {
	unsafe {
		Handle::new (PORTS_HANDLES.next ())
	}
}




static mut PROCESSES_HANDLES : PermutationCounter = PermutationCounter {
		index : 0x30df6208,
		offset : 0xa58f88be,
		initialized : false,
	};

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn processes_handles_next () -> (Handle) {
	unsafe {
		Handle::new (PROCESSES_HANDLES.next ())
	}
}

