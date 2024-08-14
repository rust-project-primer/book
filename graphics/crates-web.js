import * as d3 from "npm:d3@7";
import { JSDOM } from "npm:jsdom@24";

const { document } = (new JSDOM('')).window;

var body = d3.select(document).select("body");

// Specify the dimensions of the chart.
const width = 1024;
const height = width;
const margin = 1; // to avoid clipping the root circle stroke
const name = d => d.id;
const group = d => d.id.split(".")[1]; // "util" of "flare.util.Strings"
const names = d => name(d).split(/(?=[A-Z][a-z])|\s+/g);

const response = await fetch("https://crates.io/api/v1/crates?page=1&per_page=100&sort=recent-downloads&keyword=web");
const crates = await response.json();
const data = crates["crates"].map(crate => ({"id": crate["id"], "value": crate["downloads"]}));

// Specify the number format for values.
const format = d3.format(",d");

// Create a categorical color scale.
const color = d3.scaleOrdinal(d3.schemeTableau10);

// Create the pack layout.
const pack = d3.pack()
    .size([width - margin * 2, height - margin * 2])
    .padding(3);

// Compute the hierarchy from the (flat) data; expose the values
// for each node; lastly apply the pack layout.
const root = pack(d3.hierarchy({children: data})
    .sum(d => d.value));

// Create the SVG container.
const svg = body.append("svg")
    .attr("width", width)
    .attr("height", height)
    .attr("viewBox", [-margin, -margin, width, height])
    .attr("style", "max-width: 100%; height: auto; font: 12px sans-serif;")
    .attr("text-anchor", "middle")
    .attr("xmlns", "http://www.w3.org/2000/svg");

// Place each (leaf) node according to the layout’s x and y values.
const node = svg.append("g")
  .selectAll()
  .data(root.leaves())
  .join("g")
    .attr("transform", d => `translate(${d.x},${d.y})`);

// Add a title.
node.append("title")
    .text(d => `${d.data.id}\n${format(d.value)}`);

// Add a filled circle.
node.append("circle")
    .attr("fill-opacity", 0.7)
    .attr("fill", d => color(name(d.data)))
    .attr("r", d => d.r);

// Add a label.
const text = node.append("text")
    .attr("clip-path", d => `circle(${d.r})`);

// Add a tspan for each CamelCase-separated word.
text.selectAll()
  .data(d => names(d.data))
  .join("tspan")
    .attr("x", 0)
    .attr("y", (d, i, nodes) => `${i - nodes.length / 2 + 0.35}em`)
    .text(d => d);

// Add a tspan for the node’s value.
text.append("tspan")
    .attr("x", 0)
    .attr("y", d => `${names(d.data).length / 2 + 0.35}em`)
    .attr("fill-opacity", 0.7)
    .text(d => format(d.value));

console.log(body.node().innerHTML)

