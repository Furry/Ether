(async () => {
    let x = 0;
    let a = Array.from(document.getElementsByTagName("tr")).filter(x => {
        return x.children.length == 4
    }).filter(x => {
        return parseInt(x) != null
    })

    // Aggrigate the text value of each 3rd child of each tr
    let b = a.map(x => {
        return parseInt(x.children[2].innerText)
    })


    // Sum the values, check if nan
    let c = b.reduce((x, y) => {
        if (isNaN(y)) {
            return x
        } else {
            return x + y
        }
    }, 0)


    // Output the sum
    console.log(c)
})()