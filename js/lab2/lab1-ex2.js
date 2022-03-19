/* 
 * [2021/2022]
 * 01UDFOV Applicazioni Web I / 01TXYOV Web Applications I
 * Lab 1 - Exercise 2
 */

'use strict';
const dayjs = require("dayjs");
const sqlite = require("sqlite3");
let day = new dayjs();

const db = new sqlite.Database("films.db",
    (err) => { if (err) throw err; });

// internationalization (i18n) 
const localizedFormat = require('dayjs/plugin/localizedFormat');
dayjs.extend(localizedFormat); // use shortcuts 'LL' for date in U.S. format
/* locale
  const locale_it = require('dayjs/locale/it');
  dayjs.locale('it');
*/

function Film(id, title, isFavorite = false, watchDate = '', rating = 0) {
  this.id = id;
  this.title = title;
  this.favorite = isFavorite;
  this.rating = rating;
  // saved as dayjs object
  this.watchDate = watchDate && dayjs(watchDate).format();

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

  async function getAllDBFilms() {
    let result = [];
    return new Promise( (resolve, reject) => {
        db.all('select * from films',
            (err, rows) => {
                for(let row of rows)
                  result.push(row);
                if(err)
                     reject(err);
                else{
                    resolve(result);
                }
            }) ;            
        }) ;
  }

}


function main() {
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

  // Print Sorted films
  console.log("***** List of Films sorted by watchDate *****");
  const sorted_films = library.sortByDate();
  sorted_films.forEach((film) => console.log(film.toString()));

  // Deleting film #3
  library.deleteFilm(3);

  // Reset dates
  library.resetWatchedFilms();

  // Printing modified Library
  library.print();

  // Retrieve and print films with an assigned rating
  console.log("***** Films filtered, only the rated ones *****");
  const rated_films = library.getRated();
  rated_films.forEach((film) => console.log(film.toString()));

  // Additional instruction to enable debug 
  debugger;
  db.close();
}

async function getAllDBFilms() {
  let result = [];
  return new Promise( (resolve, reject) => {
      db.all('select * from films',
          (err, rows) => {
              for(let row of rows){
                const film = new Film(row.id, row.title, row.favorite, row.watchdate, row.rating);
                result.push(film);
              } 
              if(err)
                   reject(err);
              else{
                  resolve(result);
              }
          }) ;            
      }) ;
}



async function dbTest(){

  //const library = new FilmLibrary();
  //const add = new library.getAllDBFilms();

  let result = await getAllDBFilms();
  console.log(result);

}

dbTest();
//main();
