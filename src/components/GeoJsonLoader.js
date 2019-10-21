import React, { useState } from 'react'
import { readInChunks } from '../utils'
import ProgressBar from './ProgressBar'
import {
    PolygonDataset,
} from '/wasm/Cargo.toml';

export default function GeoJsonLoader({ onLoaded }) {
    const [progress, setProgress] = useState(0);
    const [loading, setLoading] = useState(false)
    const [file, setFile] = useState(null)

    const loadFile = async (files) => {
        setFile(files[0])
        setLoading(true)

        const geodata = await readInChunks(files[0], setProgress)

        if (onLoaded) {
            onLoaded(PolygonDataset.from_geojson(geodata))
        }
    }

    return (
        <div>
            <h1>Polygon Data</h1>
            {loading ?
                <div className={'point-loading'}>
                    <p>{file.name}: {(file.size / 1024 / 1024).toPrecision(4)} Mb</p>
                    <ProgressBar style={{ width: '60%' }} total={100.0} value={progress} />
                </div>
                :
                <React.Fragment>
                    <input accept='.geojson' type='file' name='geofile' id='geofile' onChange={(e) => loadFile(e.target.files)} className={'inputfile'} />
                    <label for='geofile'>Select GeoJSON file</label>
                </React.Fragment>
            }
        </div>
    )
}