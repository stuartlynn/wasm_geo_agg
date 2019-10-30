import React, { useEffect, useRef } from 'react'

import {
    render_points_to_canvas
} from '/wasm/Cargo.toml';

export default function PointMap({ onZoomIn, dataset, bounds }) {
    const canvasWidth = 700;
    const canvasHeight = 700;

    const canvasEl = useRef(null)

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

        console.log(canvasEl.current)
        const xPc = (e.pageX - canvasEl.current.offsetLeft) / canvasWidth;
        const yPc = (e.pageY - canvasEl.current.offsetTop) / canvasHeight;

        const xrange = (bounds[2] - bounds[0]);
        const yrange = (bounds[3] - bounds[1]);


        const xCenter = (xPc * xrange) + bounds[0];
        const yCenter = bounds[3] - (yPc * yrange);

        console.log('new x ', xPc, bounds[0], xCenter, bounds[2])
        console.log('new y ', yPc, bounds[1], yCenter, bounds[3])

        if (onZoomIn) {
            onZoomIn([
                xCenter - xrange * 0.75 / 2.0,
                yCenter - yrange * 0.75 / 2.0,
                xCenter + xrange * 0.75 / 2.0,
                yCenter + yrange * 0.75 / 2.0,
            ]);
        }
    };

    return (
        <React.Fragment>
            <h2>{dataset.no_rows.toLocaleString('en')} points</h2>
            <canvas
                ref={(ref) => canvasEl.current = ref}
                id="point_canvas"
                width={1000}
                height={1000}
                style={{ width: canvasWidth + 'px', height: canvasHeight + 'px', border: '1px solid black' }}
                onClick={zoomIn}
            />
        </React.Fragment>
    )
}