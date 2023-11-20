import { createSignal } from "solid-js";

export default function Sender() {
  const sendDatagram = async () => {
    const url = "https://127.0.0.1:4443";

    const webTransport = new WebTransport(url);
    console.log("Initiating connection...");

    try {
      await webTransport.ready;
      console.log("Connection ready");
    } catch (error) {
      console.error("Connection failed ", error);
    }

    const writer = webTransport.datagrams.writable.getWriter();

    const data = new Uint8Array([45, 65, 88]);
    await writer.write(data);

    //await writer.close();

    // webTransport.closed
    //   .then(() => {
    //     console.log("Connection closed normally.");
    //   })
    //   .catch(() => {
    //     console.error("Connection closed abruptly.", "error");
    //   });
  };

  return (
    <div>
    
      <button onClick={sendDatagram} class="increment">
        Send Datagram
      </button>
     
    </div>
  );
}
