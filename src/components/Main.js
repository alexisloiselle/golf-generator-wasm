import React, { useState } from "react";
import { useLoadedWasm } from "../useWasm";
import useWindowDimensions from "./useWindowDimensions";

const Main = () => {
  const { wasm } = useLoadedWasm();
  const { width, height } = useWindowDimensions();
  const [course, setCourse] = useState(wasm.generate_course());

  const onClick = () => {
    // Plus lent :(
    setCourse(wasm.generate_course());
  };

  return (
    <div
      onClick={onClick}
      style={{
        backgroundColor: "#6A953B",
        position: "fixed",
        top: 0,
        left: 0,
        width: "100vw",
        height: "100vh",
      }}
    >
      {[
        { outline: course.rough_outline, color: "#90C656" },
        { outline: course.fairway_outline, color: "#A5D368" },
      ].map(({ outline, color }, i) => (
        <svg
          style={{ position: "fixed", top: 0, left: 0 }}
          key={i}
          viewBox="0 0 780 780"
          width={width}
          height={height}
          fill={color}
          xmlns="http://www.w3.org/2000/svg"
        >
          <path d={`M${outline.map((p) => `${p.y} ${p.x}`).join("L")} Z`} />
        </svg>
      ))}
    </div>
  );
};

export default Main;
