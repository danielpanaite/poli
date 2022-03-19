"use strict";

const dayjs = require('dayjs');
const sqlite = require("sqlite3");

const db = new sqlite.Database("../lab2/films.db",
    (err) => { if (err) throw err; });

const localizedFormat = require('dayjs/plugin/localizedFormat');
dayjs.extend(localizedFormat); // use shortcuts 'LL' for date in U.S. format
/* locale
  const locale_it = require('dayjs/locale/it');
  dayjs.locale('it');
*/

function Film(id, title, favorites, date, rating){
    this.id = id;
    this.title = title;
    this.favorites = favorites || false;
    this.date = date;
    this.rating = rating;
}

let day = new dayjs();
let f1 = new Film(1, "Star Wars", false, undefined, 1);
let f2 = new Film(2, "Beautiful", false, day.format("2022-02-03"), 4);
let f3 = new Film(3, "Pulp Fiction", false, day.format("2022-03-10"), undefined);
let f4 = new Film(4, "Shrek\t", false, day.format("2022-03-11"), 5);

function FilmLibrary(){
    this.films = [];

    this.print = (title, films) => {
        console.log(title || "\n---- Movie Library ----");
        for(const f of films || this.films){
            console.log("Id: " + f.id + "\tTitle: " + f.title + "\t\tFavourite: " + f.favorites + "\tWatched: " + (f.date || "<not defined>") + "\tRating: " + f.rating);
        }
        console.log("\n");
    }

    this.addNewFilm = (film) => {
        this.films.push(film);
    }

    this.sortByDate = () => {
        let movies = [];

        for(const d of this.films){ //add watched films
            if(d.date != undefined){
                movies.push(d);
            }
        }

        movies.sort((a,b) => (dayjs(a.date) - (dayjs(b.date))));

        for(const d of this.films){ //add unwatched films at the end
            if(d.date == undefined){
                movies.push(d);
            }
        }

        console.log(movies);
    }

    this.deleteFilm = (film) => {
        const index = this.films.findIndex(x => x.id === film.id);
        if(index >= 0){
            this.films.splice(index,1);
        }
    }

    this.resetWatchedFilms = () => {
        for(const f of this.films){
            f.date = undefined;
        }
    }

    this.getRated = () => {
        let movies = [];

        for(const f of this.films){
            if(f.rating != undefined){
                movies.push(f);
            }
        }
        this.print("\n---- Rated films only ----", movies);
        return movies;
    }

}

let l1 = new FilmLibrary();
l1.addNewFilm(f1);
l1.addNewFilm(f2);
l1.addNewFilm(f3);
l1.addNewFilm(f4);

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

async function getAllDBFavorites() {
    let result = [];
    return new Promise( (resolve, reject) => {
        db.all('select * from films where favorite == 1',
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

async function getAllDBWatchedToday() {
    let result = [];
    return new Promise( (resolve, reject) => {
        db.all('select * from films',
            (err, rows) => {
                console.log(dayjs(rows[1].watchdate).diff(dayjs(), "minutes"));
                for(let row of rows){
                if(dayjs(row.date).diff(dayjs(), "day")){
                    const film = new Film(row.id, row.title, row.favorite, row.watchdate, row.rating);
                    result.push(film);
                }
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
  
    let result = await getAllDBWatchedToday();
    //l1.print(undefined, result);
  
}

  dbTest();