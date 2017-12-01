

#[ macro_use ]
extern crate rust_scheme;




def_tests_from_file! (
		values => "scheme/values.sst",
		types => "scheme/types.sst",
		pairs => "scheme/pairs.sst",
		lists => "scheme/lists.sst",
		quotation => "scheme/quotation.sst",
		booleans => "scheme/booleans.sst",
		control => "scheme/control.sst",
		contexts => "scheme/contexts.sst",
		lambdas => "scheme/lambdas.sst",
		functions => "scheme/functions.sst",
		functions_lists => "scheme/functions-lists.sst",
		functions_arrays => "scheme/functions-arrays.sst",
		functions_bytes => "scheme/functions-bytes.sst",
		functions_strings => "scheme/functions-strings.sst",
		comparisons_equivalent_by_identity => "scheme/comparisons-equivalent-by-identity.sst",
		comparisons_equivalent_by_value_strict => "scheme/comparisons-equivalent-by-value-strict.sst",
		comparisons_equivalent_by_value_strict_recursive => "scheme/comparisons-equivalent-by-value-strict-recursive.sst",
		comparisons_equivalent_by_value_coerced => "scheme/comparisons-equivalent-by-value-coerced.sst",
		comparisons_equivalent_by_value_coerced_recursive => "scheme/comparisons-equivalent-by-value-coerced-recursive.sst",
		arrays => "scheme/arrays.sst",
		bytes => "scheme/bytes.sst",
		strings => "scheme/strings.sst",
	);

