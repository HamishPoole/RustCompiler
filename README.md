<h2 align="center">VC Compiler</h2>
<p align="center"> A fun little Rust compiler, exploring the Rust programming language & minimalist programming.
WIP. 
</p>
<p align="center">Currently targets a subset of C called VC by Jingling Xue. 
</p>
<h2 align="center">Usage</h2>

<h3 align="center">Setup</h3>
<div class="flex-container" style="display: flex; justify-content: center;">
<div class="flex-item" style="text-align: center;">
<ul style="list-style-position: inside; display: inline-block; text-align: left;">
<li>First, clone the repository.</li>
<pre>
git@github.com:HamishPoole/RustCompiler.git
</pre>
<li>Second, build the compiler locally using build_locally.</li>
<pre>
. ./build_locally.sh
</pre>
</ul>
</div>
</div>

<h3 align="center">Running</h3>
<div class="flex-container" style="display: flex; justify-content: center;">
<div class="flex-item" style="text-align: center;">
<ul style="list-style-position: inside; display: inline-block; text-align: left;">
<li> See vc --help for the full help output.  Below is the recommended command to print a parse tree.</li>
</ul>
</div>
</div>

```
vc %path_to_valid_input_file% %path_to_valid_output_file% parse
```
