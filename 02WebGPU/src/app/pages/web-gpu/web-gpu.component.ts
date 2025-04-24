import {Component, OnInit, AfterViewInit,ViewChild,ElementRef, OnDestroy, NgZone} from '@angular/core';
import init from '../../../assets/wasm/wgpu-lib/pkg'

@Component({
  selector: 'app-web-gpu',
  templateUrl: './web-gpu.component.html',
  styleUrls: ['./web-gpu.component.sass'],
  standalone: true,
})
export class WebGPUComponent {
  ngOnInit() {
    init();
  }
}