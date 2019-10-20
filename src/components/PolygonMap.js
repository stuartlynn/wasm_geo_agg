import React, { useState, useEffect } from 'react'
import {
    render_polygons_to_canvas
} from '/wasm/Cargo.toml';

export default function PolygonMap({ dataset, bounds, counts, onZoomIn }) {

    const canvasWidth = 700;
    const canvasHeight = 700;

    useEffect(() => {
        console.log("counts are ", counts)
        dataset.display_with_counts(
            'poly_canvas',
            counts,
            bounds[0],
            bounds[1],
            bounds[2],
            bounds[3],

        )
    }, [dataset, bounds, counts])

    const zoomIn = e => {
        const x = (e.clientX * (bounds[2] - bounds[0])) / canvasWidth + bounds[0];
        const y = (e.clientY * (bounds[3] - bounds[1])) / canvasHeight + bounds[1];
        const xrange = (bounds[2] - bounds[0]) * 0.75;
        const yrange = (bounds[3] - bounds[1]) * 0.75;

        if (onZoomIn) {
            onZoomIn([
                x - xrange / 2.0,
                y + yrange / 2.0,
                x + xrange / 2.0,
                y - yrange / 2.0,
            ]);
        }
    };

    return (
        <canvas
            id="poly_canvas"
            width={1000}
            height={1000}
            style={{ width: canvasWidth, height: canvasHeight, border: '1px solid black' }}
            onClick={zoomIn}
        />

    )
}