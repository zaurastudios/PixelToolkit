import { useNavigate, useParams } from "react-router-dom";

export default function Project() {
  const { id } = useParams();
  const navigate = useNavigate();

  return <div>{id}</div>;
}
