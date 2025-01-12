// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/components/marker_shape.fbs".

#pragma once

#include "../result.hpp"

#include <cstdint>
#include <memory>

namespace arrow {
    /// \private
    template <typename T>
    class NumericBuilder;

    class Array;
    class DataType;
    class UInt8Type;
    using UInt8Builder = NumericBuilder<UInt8Type>;
} // namespace arrow

namespace rerun::components {
    /// **Component**: Shape of a marker.
    struct MarkerShape {
        uint8_t shape;

      public:
        MarkerShape() = default;

        MarkerShape(uint8_t shape_) : shape(shape_) {}

        MarkerShape& operator=(uint8_t shape_) {
            shape = shape_;
            return *this;
        }
    };
} // namespace rerun::components

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<components::MarkerShape> {
        static constexpr const char Name[] = "rerun.components.MarkerShape";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::UInt8Builder* builder, const components::MarkerShape* elements,
            size_t num_elements
        );

        /// Serializes an array of `rerun::components::MarkerShape` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::MarkerShape* instances, size_t num_instances
        );
    };
} // namespace rerun
