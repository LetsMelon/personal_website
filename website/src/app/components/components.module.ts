import { NgModule } from '@angular/core';
import { NavbarComponent } from './navbar/navbar.component';
import { LinkComponent } from './link/link.component';
import { FerrisComponent } from './ferris/ferris.component';
import { CommonModule } from '@angular/common';
import { SeparatorComponent } from './separator/separator.component';
import { FooterComponent } from './footer/footer.component';

@NgModule({
  declarations: [
    NavbarComponent,
    LinkComponent,
    FerrisComponent,
    SeparatorComponent,
    FooterComponent,
  ],
  imports: [CommonModule],
  exports: [
    NavbarComponent,
    LinkComponent,
    FerrisComponent,
    SeparatorComponent,
    FooterComponent,
  ],
})
export class ComponentsModule {}
