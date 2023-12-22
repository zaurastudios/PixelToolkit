import { Link } from "react-router-dom";

export default function Test() {
  return (
    <h1 className="text-4xl">
      hii from test
      <Link to="/" className="underliner">
        Go back home
      </Link>
    </h1>
  );
}
