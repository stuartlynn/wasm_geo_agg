
export function suggestLatitude(columns) {
  const exact = columns.find(c => c.toLocaleLowerCase() == 'latitude')
  console.log('exact is ', exact)
  if (exact) {
    return exact
  }

  const suggestions = columns.filter((c) => c.toLocaleLowerCase().includes('lat'))
  console.log("Latutude column suggestions ", suggestions)
  if (suggestions.length == 0) {
    return columns[0]
  }
  else {
    return suggestions[0]
  }
}

export function suggestLongitude(columns) {
  const exact = columns.find(c => c.toLocaleLowerCase() == 'longitude')
  if (exact) {
    return exact
  }

  const suggestions = columns.filter((c) => c.toLocaleLowerCase().includes('lon') || c.toLocaleLowerCase().includes('lng'))
  console.log("Longitude column suggestions ", suggestions)

  if (suggestions.length == 0) {
    return columns[0]
  }
  else {
    return suggestions[0]
  }
}

export function processInChunks(file, progress_callback, process) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    const chunkSize = 1024 * 10000
    let start = 0;
    let stop = chunkSize

    const loadNext = () => {
      const blob = file.slice(start, stop)
      reader.readAsText(blob)
    }

    reader.onloadend = (e) => {
      if (e.target.readyState == FileReader.DONE) {
        let chunk_string = e.target.result;
        let lines = chunk_string.split('\n')
        let last_line = lines[lines.length - 1]
        let valid_lines = chunk_string.slice(0, chunk_string.length - last_line.length - 1)
        let skip = (new TextEncoder().encode(valid_lines)).length

        start = start + skip;
        stop = start + chunkSize;

        console.log('start stop ', start, stop)
        if (progress_callback) {
          progress_callback(stop * 100.0 / file.size)
        }
        if (process) {
          process(valid_lines)
        }

        if (start + skip <= file.size) {
          loadNext()
        }
        else {
          resolve()
        }
      }
    };
    loadNext()
  })
}

export function readInChunks(file, progress_callback) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    let fileData = ""
    const chunkSize = 1024 * 1000
    let start = 0;
    let stop = chunkSize

    const loadNext = () => {
      const blob = file.slice(start, stop)
      reader.readAsText(blob)
    }

    reader.onloadend = (e) => {
      if (e.target.readyState == FileReader.DONE) {
        fileData += e.target.result;
        start = start + chunkSize;
        stop = stop + chunkSize;
        if (progress_callback) {
          progress_callback(stop * 100.0 / file.size)
        }

        if (start <= file.size) {
          loadNext()
        }
        else {
          resolve(fileData)
        }

      }
    };
    loadNext()
  })
}
