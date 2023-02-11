# cs6120-coursework

## Examples:
Generate control flow graph of Bril program as Graphviz PDF
	`cd my-bril-ext`
	`bril2json < ../bril/test/interp/core/add-overflow.bril | cargo run | dot -Tpdf -o add-overflow-cfg.pdf` 
