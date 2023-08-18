## Nx workspace with NG, Tauri, ...
https://oblique.bit.admin.ch/home.html
https://qwik.builder.io/    Qwik or Angular
https://github.com/oscartbeaumont/specta
https://json-schema.org/learn/getting-started-step-by-step
https://github.com/MatsDK/TauRPC
https://github.com/madonoharu/tsify
https://github.com/Aleph-Alpha/ts-rs
https://github.com/arabidopsis/typescript-definitions



### Nx create workspace
- Create a new nx workspace [assuming the nx cli is installed](https://nx.dev/angular-tutorial/2-project-graph)
```
create-nx-workspace gt --presete=empty
cd gt
```

### Create First Ng app and lib
- Add the nx angular plugins to the workspace
```
npm install -D @nx/angular
```
- Create the first app 
```
nx g @nx/angular:app TestPermeability  [sass,routing=false,standalone-components=yes]
```
- Create the first lib
```
nx g @nx/angular:lib ui/ng-compos
nx g @nx/angular:component banner --project=ng-compos --export -d
```
- Test and Run
```
nx lint ng-compos
nx test ng-compos
nx lint test-permeability
nx test test-permeability
nx serve test-permeability
```
- Remove ZONE
  bootstrapApplication with { provide: NgZone, useClass: ɵNoopNgZone }]   
  remove zone from polyfill 

### Add [Material](https://material.angular.io/) , [NG-ZORRO](https://ng.ant.design/docs/introduce/en)
```
npm i @angular/material
npm i ng-zorro-antd
nx g @angular/material:ng-add --project=test-permeability
```
If needed add to project.json "styles":  
```
"node_modules/ng-zorro-antd/style/index.min.css",
"node_modules/ng-zorro-antd/slider/style/index.min.css",
```

### Add [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/)

### Create a rust lib
```
nx generate @nxrs/cargo:lib rs/utils -d 
```

### Add [Tauri](https://tauri.studio/en/)
```
npm i -D @tauri-apps/cli 
npm i -D @nxrs/cargo
npm i @tauri-apps/api
cargo install tauri-cli
cd apps/test-permeability
cargo tauri init
cargo tauri dev
```
add to main Cargo.toml workspace exclude = ["apps/test-permeability/src-tauri"]


### Add [TauRPC](https://github.com/MatsDK/TauRPC)
see example https://github.com/MatsDK/threejs-tauri-test
```
npm i taurpc
cd apps/test-permeability/src-tauri
cargo add tokio
cargo add taurpc
```

