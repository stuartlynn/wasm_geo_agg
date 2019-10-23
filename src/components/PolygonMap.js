import React, { useState, useEffect } from 'react'

import InputLabel from '@material-ui/core/InputLabel';
import MenuItem from '@material-ui/core/MenuItem';
import FormControl from '@material-ui/core/FormControl';
import Select from '@material-ui/core/Select';


export default function PolygonMap({ dataset, bounds, counts, onZoomIn, columns }) {

    const canvasWidth = 700;
    const canvasHeight = 700;
    const [selectedColumn, setSelectedColumn] = useState('count')

    console.log("Columns ARE columns", columns)
    useEffect(() => {
        console.log("counts are ", counts)
        const col = selectedColumn
        let mapped_values = {}
        Object.keys(counts).forEach((regionID) => {
            mapped_values[regionID] = counts[regionID][col] ? counts[regionID][col] : 0.0
        });

        console.log('mapped values are ', mapped_values);


        dataset.display_with_counts(
            'poly_canvas',
            mapped_values,
            bounds[0],
            bounds[1],
            bounds[2],
            bounds[3],

        )
    }, [dataset, bounds, counts, selectedColumn])

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
        <React.Fragment>
            {(columns && columns.length > 0) &&
                <div id='columnSelector'>
                    <FormControl style={{ minWidth: '300px', color: 'white' }}>
                        <InputLabel style={{ color: 'white' }} htmlFor="latitude-simple">Column to show</InputLabel>
                        <Select
                            style={{ color: 'white' }}
                            value={selectedColumn}
                            onChange={(e) => { setSelectedColumn(e.target.value) }}
                        >
                            {columns.map(c =>
                                <MenuItem key={c} value={c}>{c}</MenuItem>
                            )}
                            <MenuItem key={'count'} value={'count'}>count</MenuItem>
                        </Select>
                    </FormControl>
                </div>
            }
            <canvas
                id="poly_canvas"
                width={1000}
                height={1000}
                style={{ width: canvasWidth, height: canvasHeight, border: '1px solid black' }}
                onClick={zoomIn}
            />

        </React.Fragment>

    )
}