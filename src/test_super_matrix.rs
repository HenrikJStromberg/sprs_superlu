#[cfg(test)]
mod tests {
    use crate::super_matrix::SuperMatrix;
    use sprs::{CsMat, TriMat};
    use ndarray::Array2;
    use superlu_sys::{Dtype_t, Mtype_t, Stype_t};

    extern crate superlu_sys as ffi;

    #[test]
    fn test_from_csc_mat_basic() {
        let mut tri_mat = TriMat::new((3, 3));
        tri_mat.add_triplet(0, 0, 1.0);
        tri_mat.add_triplet(1, 1, 2.0);
        tri_mat.add_triplet(2, 2, 3.0);

        let csc_mat: CsMat<f64> = tri_mat.to_csc();

        let super_matrix = SuperMatrix::from_csc_mat(csc_mat);
        unsafe {
            assert_eq!(super_matrix.nrows(), 3);
            assert_eq!(super_matrix.ncols(), 3);
            let store = &*(super_matrix.raw().Store as *const ffi::NCformat);
            assert_eq!(store.nnz, 3);
        }
    }

    #[test]
    fn test_from_csc_mat_empty() {
        let tri_mat = TriMat::new((3, 3));
        let csc_mat: CsMat<f64> = tri_mat.to_csc();

        let super_matrix = SuperMatrix::from_csc_mat(csc_mat);

        unsafe {
            assert_eq!(super_matrix.nrows(), 3);
            assert_eq!(super_matrix.ncols(), 3);
            let store = &*(super_matrix.raw().Store as *const ffi::NCformat);
            assert_eq!(store.nnz, 0);
        }
    }

    #[test]
    fn test_from_ndarray_basic() {
        use superlu_sys::{Stype_t, Dtype_t, Mtype_t};

        let array = Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap();

        let super_matrix = SuperMatrix::from_ndarray(array);

        unsafe {
            assert_eq!(super_matrix.nrows(), 2);
            assert_eq!(super_matrix.ncols(), 2);
            match super_matrix.raw().Stype {
                Stype_t::SLU_DN => {}
                _ => panic!("Stype != SLU_DN"),
            }
            match super_matrix.raw().Dtype {
                Dtype_t::SLU_D => {}
                _ => panic!("Dtype != SLU_D"),
            }
            match super_matrix.raw().Mtype {
                Mtype_t::SLU_GE => {}
                _ => panic!("Mtype != SLU_GE"),
            }
        }
    }


    #[test]
    fn test_from_ndarray_empty() {
        let array = Array2::<f64>::zeros((2, 2));

        let super_matrix = SuperMatrix::from_ndarray(array);

        unsafe {
            assert_eq!(super_matrix.nrows(), 2);
            assert_eq!(super_matrix.ncols(), 2);
            match super_matrix.raw().Stype {
                Stype_t::SLU_DN => {}
                _ => {panic!("Stype!=SLU_DN")}
            }
            match super_matrix.raw().Dtype {
                Dtype_t::SLU_D => {}
                _ => {panic!("Dtype!=SLU_D")}
            }
            match super_matrix.raw().Mtype {
                Mtype_t::SLU_GE => {}
                _ => {panic!("Mtype!=SLU_GE")}
            }
            assert!(!super_matrix.raw().Store.is_null());
        }
    }
}
