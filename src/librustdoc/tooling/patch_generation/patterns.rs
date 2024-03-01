pub enum PATTERN {
    McAdd(AddType),
    McChange(ChangeType),
}

pub enum ChangeType {
    // add -> saturating_add
    ToSaturating,
    // add -> check_add
    ToCheck,
    // add -> wrapping_add
    ToWrapping,
    // map -> filter_map
    ToFilterMap,
    // except -> unwrap
    ToUnwrap,
    // except -> unwrap_or_else
    ToUnwrapOrElse,
    // except -> unwrap_or_fault
    ToUnwrapOrFault,
    // copy_from_slice -> extend_from_slice
    ToExtendFromSlice,
}

pub enum AddType {
    // add as_bytes
    AddAsBytes,
    // add max()
    AddMax,
}
