import { interpolate } from "flubber";
import React from "react";
import { animated, Spring } from "react-spring/renderprops";

const SpringComponent = ({ from, to, style }) => {
  const interpolator = interpolate(from, to);

  return (
    <Spring reset native from={{ t: 0 }} to={{ t: 1 }}>
      {({ t }) => (
        <animated.path style={style} d={t.interpolate(interpolator)} />
      )}
    </Spring>
  );
};

export default SpringComponent;
