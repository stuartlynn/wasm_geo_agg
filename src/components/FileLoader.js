import React, { useState } from 'react'

import InputLabel from '@material-ui/core/InputLabel';
import MenuItem from '@material-ui/core/MenuItem';
import FormControl from '@material-ui/core/FormControl';
import Select from '@material-ui/core/Select';
import ProgressBar from './ProgressBar'
import { processInChunks, suggestLatitude, suggestLongitude } from '../utils'

import {
    PointDataset,
} from '/wasm/Cargo.toml';

export default function FileLoader({ type, onLoaded }) {

    const [file, setFile] = useState(null)
    const [loadPercent, setLoadPercent] = useState(0)
    const [done, setDone] = useState(false)
    const [header, setHeader] = useState(null)
    const [result, setResult] = useState(result)
    const [latitudeCol, setLatitudeCol] = useState('')
    const [longitudeCol, setLongitudeCol] = useState('')
    const [phase, setPhase] = useState('selectFile')

    const LoadHeader = (file) => {
        const reader = new FileReader();
        const chunkSize = 1024 * 1000
        reader.onloadend = (e) => {
            if (e.target.readyState == FileReader.DONE) {
                const firstLine = e.target.result.split('\n')[0]
                const columns = firstLine.split(',')
                setHeader(columns)
                setPhase('selectColumns')
                const latitudeSuggestion = suggestLatitude(columns)
                const longitudeSuggestion = suggestLongitude(columns)

                setLatitudeCol(latitudeSuggestion)
                setLongitudeCol(longitudeSuggestion)
            }
        }
        const blob = file.slice(0, chunkSize)
        reader.readAsText(blob)
    }

    const loadFile = async () => {
        const dataset = PointDataset.new_empty();
        const latColIndex = header.indexOf(latitudeCol)
        const lngColIndex = header.indexOf(longitudeCol)

        setPhase('loading')

        await processInChunks(file, setLoadPercent, (chunk) => {
            // console.log(chunk)
            // debugger
            dataset.append_csv(chunk, latColIndex, lngColIndex);
        })

        dataset.generateTree()

        if (onLoaded) {
            onLoaded(dataset)
        }
        // let dataset = new Dataset()
    }


    const gotFiles = (files) => {
        const fileToRead = files[0];
        setFile(fileToRead)
        LoadHeader(fileToRead)
    }


    return (
        <div>
            <h1>File Loader</h1>
            {phase == "selectFile" &&
                <React.Fragment>
                    <input type='file' name='csvfile' id='csvfile' onChange={(e) => gotFiles(e.target.files)} className={'inputfile'} />
                    <label for='csvfile'>Select CSV file</label>
                </React.Fragment>
            }
            {phase == "selectColumns" &&
                <div>
                    <p>{file.name}</p>
                    {header &&
                        <div>
                            <h2>Geo Columns</h2>
                            <div style={{ display: 'flex', flexDirection: 'row', justifyContent: 'space-around' }}>
                                <FormControl style={{ minWidth: '300px', color: 'white' }}>
                                    <InputLabel style={{ color: 'white' }} htmlFor="latitude-simple">Latitude Column</InputLabel>
                                    <Select
                                        style={{ color: 'white' }}
                                        value={latitudeCol}
                                        onChange={(e) => { setLatitudeCol(e.target.value) }}
                                    >
                                        {header.map(h =>
                                            <MenuItem key={h} value={h}>{h}</MenuItem>
                                        )}
                                    </Select>
                                </FormControl>
                                <FormControl style={{ minWidth: '300px', color: 'white' }}>
                                    <InputLabel style={{ color: 'white' }} htmlFor="longitude-simple">Longitude Column</InputLabel>
                                    <Select
                                        style={{ color: 'white' }}
                                        value={longitudeCol}
                                        onChange={(e) => setLongitudeCol(e.target.value)}
                                    >
                                        {header.map(h =>
                                            <MenuItem key={h} value={h}>{h}</MenuItem>
                                        )}
                                    </Select>
                                </FormControl>
                            </div>
                            <div>
                                <h2>Columns To Aggregate</h2>
                                <ul className={'columns-to-aggregate'}>
                                    {header.map((h) => <li>{h} <input type='checkbox' /></li>)}
                                </ul>
                            </div>

                            <div>
                                <button onClick={loadFile}>Load</button>
                            </div>
                        </div>
                    }
                </div>
            }
            {phase == "loading" &&
                <div className={'point-loading'}>
                    <h2>Loading</h2>
                    <p>{file.name} {(file.size / 1024 / 1024).toPrecision(5)}Mb </p>
                    <ProgressBar style={{ width: '60%' }} total={100.0} value={loadPercent} />
                </div>
            }
            {phase == 'parsing' &&
                <div>
                    <h2>Parsing</h2>
                </div>
            }
        </div>
    )
}