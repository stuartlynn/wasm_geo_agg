import Papa from 'papaparse'

export function count_csv_rows(file){
   let no_rows = 0
   let latitude = []
   let longitude = []
   return new Promise((resolve,reject)=>{
       Papa.parse(window.location.origin+file,{
         worker: true,
         download:true,
         fastMode: true,
         header: true,
         step: (row)=>{
            if(no_rows<10){
              console.log(' row number ',row)
            }
            latitude.push(parseFloat(row.data['Latitude']))
            longitude.push(parseFloat(row.data['longitude']))
            no_rows = no_rows + 1;
         },
         complete: ()=>{
           let lat_buffer = new Float32Array(latitude);
           let lng_buffer = new Float32Array(longitude);
           console.log('here ', lat_buffer, lng_buffer)
           resolve({no_rows,lat_buffer,lng_buffer})
         }
       })
    })
}
