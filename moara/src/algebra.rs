extern crate num_complex;

use num_complex::Complex32;

pub struct Matrix
{
    data:Vec<Vec<Complex32>>,
}

impl Matrix{
    pub fn new(data:Vec<Vec<Complex32>>) -> Self {

        Self {
            data:data,
            
        }
    }
    pub fn data(&self) -> &Vec<Vec<Complex32>>
    {
        &self.data
    }
    pub fn r(&self)->usize{
        return self.data.len();
    }
    pub fn c(&self)->usize{
        return self.data[0].len();
    }

}

impl Matrix{

    pub fn add(&self,m2:&Matrix)->Matrix{
        let m1=&self;
        //error
        if m1.c()!=m2.c(){
            panic!("don't match: m1{}x{} ,m2{}x{}.r",m1.c(),m1.r(),m2.c(),m2.r());
        }
        if m1.r()!=m2.r(){
            panic!("don't match: m1{}x{} ,m2{}x{}.r",m1.c(),m1.r(),m2.c(),m2.r());
        }
    
        let mut result=Matrix::new(vec![vec![Complex32::new(0.0, 0.0); m1.c()]; m1.r()]);
        for i in 0..m1.r()
        {
            for j in 0..m1.c(){
                result.data[i][j]=m1.data[i][j]+m2.data[i][j];
            }
        }
        return result ;
    }
    pub fn scalar(&self,x:Complex32)->Matrix{

        let m1=&self;
        let mut result=Matrix::new(vec![vec![Complex32::new(0.0, 0.0); m1.c()*m1.c()]; m1.r()*m1.r()]);
        for  i in 0..m1.r(){
            for j in 0..m1.c(){
                result.data[i][j]=m1.data[i][j]*x;
            }
        }
        
        return result;
    }

    pub fn dot (&self, m2:Matrix)->Matrix{
        let m1=&self;
        let mut result = Matrix::new(vec![vec![Complex32::new(0.0, 0.0); m2.c()];m1.r()]);
        //error
        if m1.c()!=m2.r(){
            panic!("don't match: m1{}x{} ,m2{}x{}.r ,{}!={}",m1.c(),m1.r(),m2.c(),m2.r(),m1.c(),m2.r());
        }
        for i in 0..m1.r()
        {
            for j in 0..m2.c()
            {
                for k in 0..m2.r()
                {
                    result.data[i][j] += m1.data[i][k]*m2.data[k][j];
                }
            }
        }
        return result;
    }
    pub fn tensor(&self, m2:&Matrix) -> Matrix {

        let m1=&self;
        let mut result = Matrix::new(vec![vec![Complex32::new(0.0, 0.0); m1.c()*m2.c()]; m2.r()*m1.r()]);
        for i in 0..m1.r(){
            for j in 0..m1.c(){
                for k in 0..m2.r(){
                    for l in 0..m2.c(){
                        result.data[i*m2.r()+k][j*m2.c()+l]=m1.data[i][j]*m2.data[k][l];
                    }
                }
            }
        }
        return result;
    }
    pub fn transpose(&self)->Matrix{

        let m=&self;
        let mut result= Matrix::new(vec![vec![Complex32::new(0.0, 0.0); m.r()]; m.c()]);
        for i in 0..result.r(){
            for j in 0..result.c(){
               result.data[i][j]=m.data[j][i];
            }
        }
        return result;
    }
//determinant
//invers

}