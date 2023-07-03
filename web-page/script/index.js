function getMetadata() {
    var file = document.getElementById("file").files[0];
    var reader = new FileReader();
    reader.readAsArrayBuffer(file);
    reader.onload = readerEvent => {
      var content = new Uint8Array(readerEvent.target.result);
      const jsonMetadata = JSON.parse(metadata_reader(content));
      printMetadataList(jsonMetadata);
    }
  }

function printMetadataList(jsonMetadata) {
    const jsonKeys = Object.keys(jsonMetadata);
    document.getElementById("result-body").innerHTML = "";
    jsonKeys.forEach(key => {
        document.getElementById("result-body").innerHTML += `<tr scope=\"row\"><th>${key}</th><td>${jsonMetadata[key]}</td></tr>`;
    });
}
