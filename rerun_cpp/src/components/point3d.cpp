// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/components/point3d.fbs"

#include "point3d.hpp"

#include "../datatypes/point3d.hpp"

#include <arrow/api.h>

namespace rr {
    namespace components {
        std::shared_ptr<arrow::DataType> Point3D::to_arrow_datatype() {
            return rr::datatypes::Point3D::to_arrow_datatype();
        }
    } // namespace components
} // namespace rr