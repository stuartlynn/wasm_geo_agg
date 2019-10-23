import React, { useEffect, useRef } from 'react'

import {
    render_points_to_canvas
} from '/wasm/Cargo.toml';

export default function PointMap({ onZoomIn, dataset, bounds }) {
    const canvasWidth = 700;
    const canvasHeight = 700;

    useEffect(() => {
        render_points_to_canvas('point_canvas',
            dataset,
            bounds[0],
            bounds[1],
            bounds[2],
            bounds[3],
            1000,
            true,
            1
        )
    }, [dataset, bounds])

    const zoomIn = e => {
        const x = (e.clientX * (bounds[2] - bounds[0])) / canvasWidth + bounds[0];
        const y = (e.clientY * (bounds[3] - bounds[1])) / canvasHeight + bounds[1];
        const xrange = (bounds[2] - bounds[0]) * 0.75;
        const yrange = (bounds[3] - bounds[1]) * 0.75;
        if (onZoomIn) {
            onZoomIn([
                x - xrange / 2.0,
                y - yrange / 2.0,
                x + xrange / 2.0,
                y + yrange / 2.0,
            ]);
        }
    };

    return (
        <React.Fragment>
            <h2>{dataset.no_rows} Rows</h2>
            <canvas
                id="point_canvas"
                width={1000}
                height={1000}
                style={{ width: canvasWidth + 'px', height: canvasHeight + 'px', border: '1px solid black' }}
                onClick={zoomIn}
            />
        </React.Fragment>
    )
}