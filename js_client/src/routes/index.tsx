import { Title } from "solid-start";
import Sender from "~/components/SendDatagram";

export default function Home() {
  return (
    <main>
      <Title>sender</Title>
      <h2>Send a datagram over webtransport!</h2>
      <Sender />
    </main>
  );
}
