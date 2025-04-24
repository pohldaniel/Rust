import {Component} from '@angular/core';
import {RouterOutlet, Router, ActivatedRoute, NavigationEnd} from '@angular/router';
import {filter, map, mergeMap} from 'rxjs/operators';
import {Observable} from "rxjs";
import {MenuComponent} from './shared/menu/menu.component';
import {ResizableDirective} from './directives/resizeable.directives';
import {CommonModule} from '@angular/common';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, MenuComponent, ResizableDirective, CommonModule],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.sass'],
})
export class AppComponent {
  showSidebar$: Observable<boolean>;
  private defaultShowSidebar = true;

  constructor(
    public router: Router,
    private activatedRoute: ActivatedRoute){
  
    //immediately logout
    this.showSidebar$ = this.router.events.pipe(
    filter(e => e instanceof NavigationEnd),
      map(() => activatedRoute),
      map(route => {      
        while (route.firstChild) {
          route = route.firstChild;
        }
        return route;
      }),
      mergeMap(route => route.data),
      map(data => data.hasOwnProperty('showSidebar') ? data['showSidebar'] : this.defaultShowSidebar),     
    )
  }  
}
