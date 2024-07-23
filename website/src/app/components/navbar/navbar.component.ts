import { Component } from '@angular/core';

@Component({
  selector: 'app-navbar',
  templateUrl: './navbar.component.html',
  styleUrl: './navbar.component.scss',
})
export class NavbarComponent {
  readonly projects = [
    {
      name: 'Home',
      link: '/',
    },
    {
      name: 'Projects',
      link: '/projects',
    },
    {
      name: 'Blogs 🚧',
      link: '/blogs',
    },
  ];
}
