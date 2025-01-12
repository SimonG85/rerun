// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/series_point.fbs".

#include "series_point.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {}

namespace rerun {

    Result<std::vector<DataCell>> AsComponents<archetypes::SeriesPoint>::serialize(
        const archetypes::SeriesPoint& archetype
    ) {
        using namespace archetypes;
        std::vector<DataCell> cells;
        cells.reserve(3);

        if (archetype.color.has_value()) {
            auto result = DataCell::from_loggable(archetype.color.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.marker.has_value()) {
            auto result = DataCell::from_loggable(archetype.marker.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto indicator = SeriesPoint::IndicatorComponent();
            auto result = DataCell::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
