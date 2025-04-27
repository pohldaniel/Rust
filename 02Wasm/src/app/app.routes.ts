import {Routes} from '@angular/router';
import {authGuard} from './guard/auth.guard';
import {WebGPUComponent} from './pages/web-gpu/web-gpu.component';
import {RustEMSComponent} from './pages/rust-ems/rust-ems.component';

export const routes: Routes = [
  { path: 'web-gpu', 
    component: WebGPUComponent,
    canActivate: [authGuard],
    data : {showSidebar: true}
  }, 
  { path: 'rust-ems', 
    component: RustEMSComponent,
    canActivate: [authGuard],
    data : {showSidebar: true},
  },
  {
    path: '**',
    redirectTo: 'rust-ems'
  } 
];
