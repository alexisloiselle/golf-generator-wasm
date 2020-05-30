const add = (p1, p2) => ({
  x: p1.x + p2.x,
  y: p1.y + p2.y,
});

const div = (p1, s) => ({
  x: p1.x / s,
  y: p1.y / s,
});

const sub = (p1, p2) => ({
  x: p1.x - p2.x,
  y: p1.y - p2.y,
});

export const generateSvgPath = (points) => {
  let path = `M ${points[0].x} ${points[0].y} `;
  const isLengthEven = points.length % 2 === 0;

  // points
  for (let i = 0; i < points.length - (isLengthEven ? 3 : 2); i += 2) {
    const p1 = points[i + 2];
    const midPoint = div(add(points[i], points[i + 2]), 2);
    const p0 = add(points[i + 1], sub(points[i + 1], midPoint));
    path = `${path} Q ${p0.x} ${p0.y} ${p1.x} ${p1.y} `;
  }

  //  last
  if (isLengthEven) {
    const p1 = points[0];
    const midPoint = div(add(points[points.length - 2], points[0]), 2);
    const p0 = add(
      points[points.length - 1],
      sub(points[points.length - 1], midPoint)
    );
    path = `${path} Q ${p0.x} ${p0.y} ${p1.x} ${p1.y}`;
  }

  return path;
};
