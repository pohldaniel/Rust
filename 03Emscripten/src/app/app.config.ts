import {ApplicationConfig, provideZoneChangeDetection, importProvidersFrom, LOCALE_ID, ErrorHandler} from '@angular/core';
import {provideRouter, withComponentInputBinding, withHashLocation} from '@angular/router';
import {BrowserAnimationsModule} from '@angular/platform-browser/animations';
import {IMAGE_CONFIG} from '@angular/common';
import {provideHttpClient, withInterceptors} from '@angular/common/http';
import {routes} from './app.routes';
import {intercept} from './interceptor/session.interceptor';

export const appConfig: ApplicationConfig = {
  providers: [
    provideZoneChangeDetection({ eventCoalescing: true }), 
    provideRouter(routes, withComponentInputBinding()),
    importProvidersFrom(BrowserAnimationsModule),
    provideHttpClient(withInterceptors([intercept])),
    {provide: IMAGE_CONFIG, useValue: {disableImageSizeWarning: true}},
    {provide: LOCALE_ID, useValue: 'de-DE'},
    //{provide: ErrorHandler, useClass: GlobalErrorHandler},
  ]
};
