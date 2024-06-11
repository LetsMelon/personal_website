import { Component } from '@angular/core';

@Component({
  selector: 'app-footer',
  templateUrl: './footer.component.html',
  styleUrl: './footer.component.scss',
})
export class FooterComponent {
  readonly furtherLinks: { link: string; name: string }[] = [
    { link: 'https://github.com/LetsMelon', name: 'GitHub' },
    { link: 'https://www.linkedin.com/in/domenic-melcher/', name: 'LinkedIn' },
  ];
}
