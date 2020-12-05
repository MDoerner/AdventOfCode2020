import Day from "./Day";

interface TravelDocument{
    byr?: string;
    iyr?: string;
    eyr?: string;
    hgt?: string;
    hcl?: string;
    ecl?: string;
    pid?: string;
    cid?: string;
}

interface Passport{
    byr: string;
    iyr: string;
    eyr: string;
    hgt: string;
    hcl: string;
    ecl: string;
    pid: string;
    cid?: string;
}


class Day4 implements Day<TravelDocument[]>{
    parseInput(text: string): TravelDocument[] {
        let documentTexts = text.split(/\r?\n\r?\n/);
        return documentTexts.map(((documentText: string) => this.readDocument(documentText)));
    }

    private readDocument(text: string): TravelDocument{
        return text.replace(/\r?\n/g, ' ')
            .split(' ')
            .map((documentItem: string) => documentItem.split(':'))
            .reduce((document: TravelDocument, item: [string, string]) => {
                    document[item[0]] = item[1];
                    return document;
                }, {});
    }

    solvePart1(input: TravelDocument[]): string {
        return input.filter((document: TravelDocument) => this.isPassport(document))
            .length
            .toString();
    }

    private isPassport(document: TravelDocument): boolean{
        return 'byr' in document
            && 'iyr' in document
            && 'eyr' in document
            && 'hgt' in document
            && 'hcl' in document
            && 'ecl' in document
            && 'pid' in document;
    }

    solvePart2(input: TravelDocument[]): string {
        return input.filter((document: TravelDocument) => this.isPassport(document))
            .map((document: TravelDocument) => document as Passport)
            .filter((document: Passport) => this.isValid(document))
            .length
            .toString();
    }

    private isValid(passport: Passport): boolean{
        return this.isValidBirthYear(passport.byr)
            && this.isValidIssueYear(passport.iyr)
            && this.isValidExpirationYear(passport.eyr)
            && this.isValidHeight(passport.hgt)
            && this.isValidHairColor(passport.hcl)
            && this.isValidEyeColor(passport.ecl)
            && this.isValidPassportId(passport.pid);
    }

    private isValidBirthYear(text: string): boolean{
        return this.isValidYear(text, 1920, 2002);
    }

    private isValidYear(text: string, minYear: number, maxYear: number): boolean{
        if(!/^\d{4}$/.test(text)){
            return false;
        }

        let year: number = parseInt(text);
        return minYear <= year
            && year <= maxYear;
    }

    private isValidIssueYear(text: string): boolean{
        return this.isValidYear(text, 2010, 2020);
    }

    private isValidExpirationYear(text: string): boolean{
        return this.isValidYear(text, 2020, 2030);
    }

    private isValidHeight(text: string): boolean{
        let match = text.match(/^(\d+)(in|cm)$/);

        if(!match){
            return false;
        }

        let heightValue: number = parseInt(match[1]);

        switch(match[2]){
            case 'cm': return 150 <= heightValue && heightValue <= 193;
            case 'in': return 59 <= heightValue && heightValue <= 76;
            default: return false;
        }
    }

    private isValidHairColor(text: string): boolean{
        return /^#[0-9a-f]{6}$/.test(text);
    }

    private readonly validEyeColors = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'];
    private isValidEyeColor(text: string): boolean{
        return this.validEyeColors.includes(text);
    }

    private isValidPassportId(text: string): boolean{
        return /^\d{9}$/.test(text);
    }
}

export default Day4;