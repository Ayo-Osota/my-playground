const handlePrint = () => {
    const printContents = document.getElementById("printable").innerHTML;

    // Create a hidden iframe
    const iframe = document.createElement("iframe");
    iframe.srcdoc = "<!DOCTYPE html>";
    iframe.style.position = "absolute";
    iframe.style.left = "-9999px"; // Hide the iframe

    // Append the iframe to the body
    document.body.appendChild(iframe);

    // Write the content to the iframe's document
    var doc = iframe.contentWindow?.document;
    doc?.open();
    doc?.write(`
    <!DOCTYPE html>
          <html class="print-frame">
            <head>
            <link rel="stylesheet" href="style.css">
            <title> </title>
            </head>
            <body class="printarea">
              ${printContents}
            </body>
          </html>
        `);
    doc?.close();

    const printWindow = iframe.contentWindow;

    if (printWindow) {
        // Set focus on the iframe and trigger print
        printWindow.focus();
        printWindow.print();

        // Add an event listener for afterprint to clean up and send the estimate
        printWindow.addEventListener("afterprint", () => {
            // Clean up the iframe after printing
            document.body.removeChild(iframe);

            // Call the sendEstimate function

        });
    }
};

