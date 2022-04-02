"use strict";

function Film(id, title, isFavorite = false, watchDate = '', rating = 0) {
    this.id = id;
    this.title = title;
    this.favorite = isFavorite;
    this.rating = rating;
    // saved as dayjs object
    this.watchDate = watchDate && dayjs(watchDate);
  
    this.toString = () => {
      return `Id: ${this.id}, ` +
      `Title: ${this.title}, Favorite: ${this.favorite}, Score: ${this._formatRating()}, ` +
      `watchDate: ${this._formatWatchDate('LL')}`;
    }
  
    this._formatWatchDate = (format) => {
      return this.watchDate ? this.watchDate.format(format) : '<not defined>';
    }
  
    this._formatRating = () => {
      return this.rating ? this.rating : '<not assigned>';
    }
  }
  
function FilmLibrary() {
    this.list = [];

    this.print = () => {
        console.log("***** List of Films *****");
        this.list.forEach((item) => console.log(item.toString()));
    }

    this.addNewFilm = (film) => {
        if(!this.list.some(f => f.id == film.id))
        this.list.push(film);
        else
        throw new Error('Duplicate id');
    };

    this.deleteFilm = (id) => {
        const new_list = this.list.filter(function(film, index, arr) {
        return film.id !== id;
        })
        this.list = new_list;
    }

    this.resetWatchedFilms = () => {
        this.list.forEach((film) => film.watchDate = '');
    }

    this.getRated = () => {
        const new_list = this.list.filter(function(film, index, arr) {
        return film.rating > 0;
        })
        return new_list;
    }

    this.sortByDate = () => {
        const new_array = [...this.list];
        new_array.sort((f1, f2) => {
        if(f1.watchDate === f2.watchDate)
            return 0;    // works also for null === null
        else if(f1.watchDate === null || f1.watchDate === '')
            return 1;    // null/empty watchDate is the lower value
        else if(f2.watchDate === null || f2.watchDate === '')
            return -1;
        else
            return f1.watchDate.diff(f2.watchDate)
        });
        return new_array;
    }
}

  
// Creating some film entries
const f1 = new Film(1, "Pulp Fiction", true, "2022-03-10", 5);
const f2 = new Film(2, "21 Grams", true, "2022-03-17", 4);
const f3 = new Film(3, "Star Wars", false);
const f4 = new Film(4, "Matrix", false);
const f5 = new Film(5, "Shrek", false, "2022-03-21", 3);

// Adding the films to the FilmLibrary
const library = new FilmLibrary();
library.addNewFilm(f1);
library.addNewFilm(f2);
library.addNewFilm(f3);
library.addNewFilm(f4);
library.addNewFilm(f5);

const table = document.querySelector('tbody');
table.innerHTML = ``;
const row = document.createElement('tr');
row.innerHTML=`<th>Title</th>
    <th>Favourite</th>
    <th>Watch Date</th>
    <th>Rating</th>`;
table.appendChild(row);
const films = library.sortByDate();

for(const f of films){
    const newrow = document.createElement('tr');
    newrow.innerHTML = `<td>${f.title}</td>
        <td>${f.favorite}</td>
        <td>${f.watchDate}</td>
        <td>${f.rating}</td>`;
    table.appendChild(newrow);
} 