import {Directive, ElementRef, OnInit, Input, HostListener} from '@angular/core';

@Directive({
  standalone:true,
  selector: '[appResizeable]'
})
export class ResizableDirective implements OnInit {
  @Input() resizeableGrabWidth = 0;
  @Input() resizeableMinWidthA = 50;
  @Input() resizeableMaxWidthA = window.innerWidth;
  @Input() resizeableWidthA = 0;

  dragging = false;

  windowWidth: number = 0;
  storedWidth: number = 0;

  @HostListener('window:resize', ['$event'])
  onResize(event: any) {
    this.windowWidth = event.target.innerWidth;   
    //this.resizableMaxWidthA = this.windowWidth;
    this._el.nativeElement.children[1].style.width = (this.windowWidth - (this.storedWidth + 0.5)) + "px";

    /*if(this.storedWidth > this.windowWidth) {
      this.storedWidth = this.windowWidth;
      this._el.nativeElement.children[0].style.width = this.windowWidth;
    }*/
  }

  constructor(private _el: ElementRef) {
    
    const self = this;
    const EventListenerMode = { capture: true };


    function restoreGlobalMouseEvents() {
      document.body.style['pointer-events'as any] = 'auto';
    }

    const newWidth = (wid: any) => {
      let newWidth = Math.max(this.resizeableMinWidthA, wid);  
      newWidth = Math.min(this.resizeableMaxWidthA, newWidth); 
      this.storedWidth = newWidth;
      _el.nativeElement.children[0].style.width = (newWidth) + "px";
      _el.nativeElement.children[1].style.width = (this.windowWidth - (this.storedWidth + 0.5)) + "px";
    }

    const mouseMoveG = (evt: any) => {
      if (!this.dragging) {
        return;
      }

      newWidth((evt.clientX)  - (_el.nativeElement.children[0].offsetLeft))
      evt.stopPropagation();
    };
  
    const mouseUpG = (evt: any) => {
      if (!this.dragging) {
        return;
      }
      restoreGlobalMouseEvents();
      this.dragging = false;
      evt.stopPropagation();
    };

    document.addEventListener('mousemove', mouseMoveG, true);
    document.addEventListener('mouseup', mouseUpG, true);
   
  }

  update(){

    this._el.nativeElement.children[0].style.width = 200 + "px"
    this._el.nativeElement.children[1].style.width = (this.windowWidth - (200 + 0.5)) + "px";
  }

  ngOnInit(): void {
  
    this.windowWidth = this._el.nativeElement.clientWidth;
    this.storedWidth = this.resizeableWidthA;

    this._el.nativeElement.children[0].style["border-right"] = this.resizeableGrabWidth + "px solid darkgrey";
    this._el.nativeElement.children[0].style.width = this.resizeableWidthA + "px";
    this._el.nativeElement.children[1].style.width = (this.windowWidth - (this.storedWidth + 0.5)) + "px";
  }

  inDragRegion(evt: any) {
    return this._el.nativeElement.children[0].clientWidth - evt.clientX + this._el.nativeElement.children[0].offsetLeft < this.resizeableGrabWidth;
  }

  ngAfterViewInit() {

    function preventGlobalMouseEvents() {
      document.body.style['pointer-events' as any] = 'none';
    }

    const mouseDown = (evt: any) => {
      if (this.inDragRegion(evt)) {
        this.dragging = true;
        preventGlobalMouseEvents();
        evt.stopPropagation();
      }
    };

    const mouseMove = (evt: any) => {
      if (this.inDragRegion(evt) || this.dragging) {
        this._el.nativeElement.children[0].style["cursor"]  = "col-resize";
      } else {
        this._el.nativeElement.children[0].style["cursor"]  = "default";
      }
    }

    this._el.nativeElement.children[0].addEventListener('mousedown', mouseDown, true);
    this._el.nativeElement.children[0].addEventListener('mousemove', mouseMove, true);
  }
}
