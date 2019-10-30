import React, { useState, useEffect, useRef } from 'react'

import InputLabel from '@material-ui/core/InputLabel';
import MenuItem from '@material-ui/core/MenuItem';
import FormControl from '@material-ui/core/FormControl';
import Select from '@material-ui/core/Select';
// import Select from 'react-select';


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

    const canvasEl = useRef(null)


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

    const customStyles = {
        input: (provided) => ({ ...provided, backgroundColor: 'green' })
    }

    return (
        <React.Fragment>
            <div class='polygon-map-header'>
                <h2>{dataset.no_objects.toLocaleString('en')} Polygons</h2>

                {(columns && columns.length > 0) &&

                    <div id='columnSelector'>
                        <FormControl style={{ minWidth: '300px', color: 'white' }}>
                            <InputLabel style={{ color: 'white' }} htmlFor="latitude-simple">Column to show</InputLabel>
                            <Select
                                style={{ color: 'white' }}
                                value={selectedColumn}
                                onChange={(e) => { setSelectedColumn(e.target.value) }}
                            >
                                {[...columns.map(c =>
                                    <MenuItem key={c} value={c}>{c}</MenuItem>
                                ), ...columns.map(c => <MenuItem key={`${c}_avg`} value={`${c}_avg`}>{`${c}_avg`}</MenuItem>)
                                ]}
                                <MenuItem key={'count'} value={'count'}>count</MenuItem>
                            </Select>
                        </FormControl>


                    </div>
                }
            </div>
            <canvas
                ref={ref => canvasEl.current = ref}
                id="poly_canvas"
                width={1000}
                height={1000}
                style={{ width: canvasWidth, height: canvasHeight, border: '1px solid black' }}
                onClick={zoomIn}
            />

        </React.Fragment>

    )
}