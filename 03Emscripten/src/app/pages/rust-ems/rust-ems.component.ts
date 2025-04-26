import {Component, OnInit, AfterViewInit,ViewChild,ElementRef, OnDestroy, NgZone} from '@angular/core';
import {EmscriptenWasmComponent} from 'src/app/emscripten-wasm.component';

@Component({
  selector: 'app-rust-ems-cpp',
  templateUrl: './rust-ems.component.html',
  styleUrls: ['./rust-ems.component.sass'],
  standalone: true,
})
export class RustEMSComponent extends EmscriptenWasmComponent {
  @ViewChild("canvas") canvas!: ElementRef;
  error!: string;
  constructor(private ngZone: NgZone) {
    super('RustEMSModule',  'assets/wasm/ems-lib/dist/rust.js', 'assets/wasm/ems-lib/dist/rust.wasm', 'assets/wasm/ems-lib/dist/rust.data');
    this.moduleDecorator = (mod) => {     
      mod.canvas = <HTMLCanvasElement>this.canvas.nativeElement;
      mod.printErr = (what: string) => {
        if (!what.startsWith("WARNING")) {
          this.ngZone.run(() => (this.error = what));
        }
      };
    };
  }
}