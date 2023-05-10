const BITS: u32 = 32; // Константа, которая определяет количество бит в одном числе

#[derive(Debug)]
struct BigNumber {
    data: Vec<u64>, // Массив беззнаковых целых чисел, который представляет большое число
    hex: String, // Шестнадцатеричное представление большого числа
}

impl Clone for BigNumber {
    fn clone(&self) -> Self { // Реализация клонирования структуры BigNumber
        BigNumber {
            data: self.data.clone(), // Клонирование массива данных
            hex: self.hex.clone(), // Клонирование шестнадцатеричного представления
        }
    }
}

impl BigNumber {
    fn new() -> Self {  // Создание новой структуры BigNumber
        Self {         // Возвращает новую структуру с массивом данных из одного элемента и пустым шестнадцатеричным представлением
            data: vec![0],
            hex: String::new(),
        }
    }

    fn set_hex(&mut self, hex: &str) {     // Метод установки шестнадцатеричного представления числа
        self.hex = hex.to_string();         // Сохранение шестнадцатеричного представления числа
        let mut temp_data = Vec::new();     // Создание временного массива данных
        let mut slice = &self.hex[..];     // Создание ссылки на подстроку строки с шестнадцатеричным числом

        // Добавление частей шестнадцатеричного числа по 16 символов в массив чисел
        while !slice.is_empty() {
            let len = std::cmp::min(slice.len(), 16);
            let s = &slice[..len];
            slice = &slice[len..];
            let val = u64::from_str_radix(s, 16).unwrap();
            temp_data.push(val);
        }
        self.data = temp_data;  // Сохранение массива чисел
    }

    fn get_hex(&self) -> String {   // Метод получения шестнадцатеричного представления числа
        self.hex.clone()     // Возвращает копию шестнадцатеричного представления
    }
}

#[derive(Clone)]
enum Sign {
    Positive,  // Положительный знак
    Negative,  // Отрицательный знак
}

#[derive(Clone)]
pub struct MyBigInt {
    sign: Sign,  // Знак числа
    digits: Vec<u32>,  // Вектор цифр числа в порядке от младших разрядов к старшим
    other: BigNumber,  // Вспомогательное поле для хранения числа в виде строки шестнадцатеричных цифр
}

impl Default for MyBigInt {
    fn default() -> Self {
        Self {
            digits: vec![0],  // По умолчанию число равно нулю
            sign: Sign::Positive, // По умолчанию число положительное
            other: BigNumber {
                data: vec![],
                hex: String::new(),
            },
        }
    }
}

impl MyBigInt {  // Определение структуры MyBigInt
    pub fn set_hex(&mut self, hex: &str) {  // Публичный метод set_hex, который принимает строку hex и изменяет значение поля other структуры
        self.other.set_hex(hex);
    }

    pub fn get_hex(&self) -> String { // Публичный метод get_hex, который возвращает значение поля other структуры в виде строки
        self.other.get_hex()
    }

    fn trim(&mut self) {   // Приватный метод trim, который удаляет нулевые элементы из конца вектора digits структуры
        while let Some(&0) = self.digits.last() {
            self.digits.pop();
        }
        if self.digits.is_empty() {    // Если вектор digits пустой, то добавляется элемент 0
            self.digits.push(0);
        }
    }

    pub fn new() -> MyBigInt {
    MyBigInt {
        digits: Vec::new(), // создаем новый вектор для хранения цифр
        sign: Sign::Positive, // устанавливаем знак на положительный
        other: BigNumber { // создаем структуру BigNumber, связанную с MyBigInt
            data: vec![],
            hex: String::new(),
        },
    }
}

// побитовая инверсия
pub fn inv(&mut self) {
    for i in 0..self.digits.len() { // проходимся по каждому элементу вектора digits
        self.digits[i] = !self.digits[i]; // инвертируем значение каждого элемента вектора digits
    }
}

// побитовое исключительное ИЛИ
pub fn xor(&self, other: &MyBigInt) -> MyBigInt {
    let mut result = MyBigInt::default(); // создаем новый экземпляр MyBigInt с пустым вектором digits и положительным знаком
    let len = self.digits.len().max(other.digits.len()); // определяем длину, на которую необходимо продлить вектор digits в случае, если одно из чисел короче другого

    for i in 0..len { // проходимся по каждому элементу вектора digits в диапазоне от 0 до len
        let a = *self.digits.get(i).unwrap_or(&0); // получаем значение i-го элемента вектора digits экземпляра self
        let b = *other.digits.get(i).unwrap_or(&0); // получаем значение i-го элемента вектора digits экземпляра other
        result.digits.push(a ^ b); // выполняем побитовое исключающее ИЛИ между a и b и добавляем результат в вектор digits результата
    }

    // добавляем оставшиеся элементы более длинного числа в вектор digits результата
    for i in len..self.digits.len() {
        result.digits.push(self.digits[i]);
    }

    for i in len..other.digits.len() {
        result.digits.push(other.digits[i]);
    }

    result.trim(); // удаляем ведущие нули из вектора digits результата
    result // возвращаем экземпляр MyBigInt с новым вектором digits
}

    // побитовое ИЛИ
    pub fn or(&self, other: &MyBigInt) -> MyBigInt {
        let mut result = MyBigInt::default();
        let len = self.digits.len().max(other.digits.len()); // Вычисляем длину, которая будет использоваться при итерировании цикла

        // Цикл проходится по каждому биту чисел, выполняет операцию "ИЛИ" и записывает результат в новый MyBigInt
        for i in 0..len {
          let a = *self.digits.get(i).unwrap_or(&0); // Берем i-й элемент из вектора, если его нет, то используем 0
          let b = *other.digits.get(i).unwrap_or(&0); // Аналогично для второго числа
          result.digits.push(a | b); // Добавляем результат операции "ИЛИ" в новый вектор
         }

        result.trim(); // Отрезаем старшие нули от результата
        result // Возвращаем результат

    }

    // побитовое И
    pub fn and(&self, other: &MyBigInt) -> MyBigInt {
        let mut result = MyBigInt::default();
        let len = self.digits.len().max(other.digits.len()); / Вычисляем длину, которая будет использоваться при итерировании цикла

        // Цикл проходится по каждому биту чисел, выполняет операцию "И" и записывает результат в новый MyBigInt
          for i in 0..len {
              let a = *self.digits.get(i).unwrap_or(&0); // Берем i-й элемент из вектора, если его нет, то используем 0
              let b = *other.digits.get(i).unwrap_or(&0); // Аналогично для второго числа
              result.digits.push(a & b); // Добавляем результат операции "И" в новый вектор
          }

        result.trim(); // Отрезаем старшие нули от результата
        result // Возвращаем результат

    }

    // сдвиг в право на n битов
    pub fn shift_r(&self, n: u32) -> MyBigInt {
        let mut result = MyBigInt::default();
        let mut carry = 0u32;

        // Проходим по всем цифрам числа
        for i in 0..self.digits.len() {
            let x = u32::from(self.digits[i]); // конвертируем в u32
            // Сдвигаем текущую цифру вправо на n бит и применяем битовое ИЛИ с предыдущим carry
            let y = (x >> n) | carry;
            // Сохраняем остаток от сдвига текущей цифры в carry
            carry = x << (32 - n);
            // Добавляем новую цифру к результату
            result.digits.push(y as u32);
        }

        // Если после цикла остался непустой carry, добавляем его как новую цифру
        if carry != 0 {
            result.digits.push(carry as u32);
        }

        // Обрезаем лидирующие нули
        result.trim();

        // Возвращаем результат
        result
    }


    // сдвиг в лево на n битов
    pub fn shift_l(&self, n: u32) -> MyBigInt {
        let mut result = MyBigInt::default();
        let mut carry = 0;

        // Итерируем цифры числа в обратном порядке
        for i in (0..self.digits.len()).rev() {
            let x = self.digits[i];
            // Сдвигаем текущую цифру влево на n бит и применяем битовое ИЛИ с предыдущим carry
            let y = (x << n) | carry;
            // Сохраняем остаток от сдвига текущей цифры в carry
            carry = x >> (BITS - n);
            // Добавляем новую цифру к результату в начало вектора
            result.digits.insert(0, y);
        }

        // Если после цикла остался непустой carry, добавляем его как новую цифру
        if carry != 0 {
            result.digits.insert(0, carry);
        }

        // Обрезаем лидирующие нули
        result.trim();

        // Возвращаем результат
        result
    }

    // ADD 
    pub fn add(&self, other: &MyBigInt) -> MyBigInt {
        // Создаем новый MyBigInt результат.
        let mut result = MyBigInt::default();
        // Определяем максимальную длину двух чисел для определения количества итераций в цикле.
        let len = self.digits.len().max(other.digits.len());

        // carry - это переменная для хранения переноса из предыдущего разряда при сложении.
        let mut carry = 0;
        // Цикл, который проходит по всем разрядам двух чисел и складывает их.
        for i in 0..len {
            // Получаем значение i-го разряда в каждом из чисел.
            let a = *self.digits.get(i).unwrap_or(&0);
            let b = *other.digits.get(i).unwrap_or(&0);
            // Складываем значения разрядов, а также значение переноса из предыдущего разряда.
            let sum = a as u64 + b as u64 + carry as u64;
            // Вычисляем новый перенос.
            carry = (sum >> BITS) as u32;
            // Добавляем остаток от суммы разрядов (без учета переноса) в результат.
            result.digits.push((sum & ((1 << BITS) - 1)) as u32);
        }

        // Если после последнего разряда остался перенос, добавляем его в результат.
        if carry > 0 {
            result.digits.push(carry);
        }

        // Убираем ведущие нули в результате.
        result.trim();
        // Вычисляем знак результата.
        result.sign = if self.sign == other.sign {
            self.sign.clone()
        } else if self.abs() > other.abs() {
            self.sign.clone()
        } else {
            other.sign.clone()
        };

        // Возвращаем результат.
        result
    }

    // SUB 
    pub fn sub(&self, other: &MyBigInt) -> MyBigInt { // Определение метода sub с двумя параметрами типа MyBigInt и возвратом MyBigInt
        let mut result = MyBigInt::default(); // Создание нового экземпляра MyBigInt с помощью конструктора MyBigInt::default()
        let len = self.digits.len().max(other.digits.len()); // Находим максимальную длину между digits текущего экземпляра и digits другого экземпляра

        let mut borrow = 0; // Инициализируем значение "заема" как 0
            for i in 0..len { // Итерируем от 0 до len
                let a = *self.digits.get(i).unwrap_or(&0); // Получаем цифру текущего экземпляра MyBigInt по индексу i, или 0, если такого индекса нет.
                let b = *other.digits.get(i).unwrap_or(&0); // Получаем цифру другого экземпляра MyBigInt по индексу i, или 0, если такого индекса нет.
                let diff = a as i64 - b as i64 - borrow as i64; // Вычисляем разность между цифрами с учетом займа.
                borrow = if diff < 0 { 1 } else { 0 }; // Если разность меньше нуля, то значение займа увеличивается на 1.
                result.digits.push((diff & ((1 << BITS) - 1)) as u32); // Записываем последнюю цифру разности в digits результата.
            }

            if borrow > 0 { // Если значение займа больше нуля, то добавляем цифру 0 в digits результата.
                result.digits.push(0);
            }

            result.trim(); // Удаляем нули в начале digits результата.
            result.sign = if self.abs() >= other.abs() { // Определяем знак результата на основе значений self и other, используя метод abs() (абсолютное значение).
                self.sign.clone() // Если self.abs() >= other.abs(), то знак результата равен знаку self.
            } else {
                self.sign.clone().swap() // Иначе, знак результата равен обратному знаку self.
            };

        result // Возвращаем результат.
      }


    // MOD 
    pub fn modulus(&self, other: &MyBigInt) -> MyBigInt {
        let mut dividend = self.abs(); // Получаем модуль делимого
        let divisor = other.abs(); // Получаем модуль делителя

        let mut quotient = MyBigInt::default(); // Инициализируем частное нулем
        let mut remainder = MyBigInt::default(); // Инициализируем остаток нулем

        // Проходим по всем цифрам делимого в обратном порядке
        for i in (0..dividend.digits.len()).rev() {
            remainder.digits.insert(0, dividend.digits[i]); // Добавляем текущую цифру в начало остатка

            let mut x = MyBigInt::from(0); // Инициализируем переменную-множитель нулем
            let mut left = 0; // Инициализируем левую границу диапазона значений множителя
            let mut right = (1 << BITS) - 1; // Инициализируем правую границу диапазона значений множителя

            // Бинарный поиск множителя, который является наибольшим числом, удовлетворяющим условию product <= remainder
            while left <= right {
                let mid = (left + right) / 2; // Вычисляем среднее значение между левой и правой границей
                x.digits = vec![mid]; // Задаем множитель x

                let product = divisor.mul(&x); // Умножаем делитель на множитель x

                if product <= remainder { // Если произведение меньше или равно остатку
                    quotient = quotient.mul(&MyBigInt::from((1 << BITS))); // Увеличиваем частное на 2^BITS
                    quotient = quotient.add(&x); // Добавляем к частному множитель x
                    remainder = remainder.sub(&product); // Вычитаем из остатка произведение делителя на множитель x
                    break; // Прекращаем бинарный поиск
                } else {
                    right = mid - 1; // Смещаем правую границу диапазона значений множителя
                }
            }
        }

        quotient.trim(); // Удаляем ведущие нули из частного
        quotient.sign = if self.sign == other.sign {
            self.sign.clone() // Если знаки делимого и делителя совпадают, то знак частного будет положительным
        } else {
            Sign::Negative // Иначе знак частного будет отрицательным
        };

        quotient // Возвращаем частное
    }
}
