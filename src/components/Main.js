import React, { useEffect, useRef, useState } from "react";
import { useLoadedWasm } from "../useWasm";
import SpringComponent from "./SpringComponent";
import useWindowDimensions from "./useWindowDimensions";

function usePrevious(value) {
  const ref = useRef();
  useEffect(() => {
    ref.current = value;
  });
  return ref.current;
}

const getPolePath = (point) => `M ${point.x} ${point.y} l 0 -45`;
const getFlagPath = (point) => `M ${point.x} ${point.y - 45} l 18 6 l -18 6 z`;
const getGreenPath = ({ x: gX, y: gY }) =>
  `M ${gX - 30} ${gY} a 30 30 0 1 0 60 0 a 30 30 0 1 0 -60 0`;
const getHole = ({ x: gX, y: gY }) =>
  `M ${gX - 2.5} ${gY} a 2.5 2.5 0 1 0 5 0 a 2.5 2.5 0 1 0 -5 0`;
const getTeePath = ({ x: teeX, y: teeY }) =>
  `M ${teeX - 20} ${teeY} a 20 20 0 1 0 40 0 a 20 20 0 1 0 -40 0`;
const getTee = ({ x: teeX, y: teeY }) =>
  `M ${teeX - 5} ${teeY} a 5 5 0 1 0 10 0 a 5 5 0 1 0 -10 0`;

const Main = () => {
  const { wasm } = useLoadedWasm();
  const { width, height } = useWindowDimensions();
  const [course, setCourse] = useState(wasm.generate_svg_course());
  const previousCourse = usePrevious(course) || course;

  const onClick = () => setCourse(wasm.generate_svg_course());
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
        {
          path: course.rough_svg,
          prev: previousCourse.rough_svg,
          color: "#90C656",
        },
        {
          path: course.fairway_svg,
          prev: previousCourse.fairway_svg,
          color: "#A5D368",
        },
        {
          path: getGreenPath(course.green_center),
          prev: getGreenPath(previousCourse.green_center),
          color: "#B0DA74",
        },
        {
          path: getTeePath(course.tee_center),
          prev: getTeePath(previousCourse.tee_center),
          color: "#B0DA74",
        },
        {
          path: getHole(course.green_center),
          prev: getHole(previousCourse.green_center),
          color: "#416320",
        },
        {
          path: getTee(course.tee_center),
          prev: getTee(previousCourse.tee_center),
          color: "#A5D368",
        },
        {
          path: getPolePath(course.green_center),
          prev: getPolePath(previousCourse.green_center),
          color: "none",
          style: { strokeWidth: 2, stroke: "black" },
        },
        {
          path: getFlagPath(course.green_center),
          prev: getFlagPath(previousCourse.green_center),
          color: "none",
          style: { fill: "#DD4D4B" },
        },
        {
          path: getFlagPath(course.green_center),
          prev: getFlagPath(previousCourse.green_center),
          color: "none",
          style: { stroke: "#DD4D4B", strokeLinejoin: "round", strokeWidth: 2 },
        },
      ].map(({ path, prev, color, style }, i) => (
        <svg
          style={{ position: "fixed", top: 0, left: 0 }}
          key={i}
          viewBox="0 0 780 780"
          width={width}
          height={height}
          fill={color}
          xmlns="http://www.w3.org/2000/svg"
        >
          <SpringComponent from={prev} to={path} style={style} />
        </svg>
      ))}
    </div>
  );
};

export default Main;
