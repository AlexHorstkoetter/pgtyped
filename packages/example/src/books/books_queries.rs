/** Types generated for queries found in "src/books/books.sql" */
use @pgtyped/query::{ PreparedQuery };

pub type StringArray = Vec<String>;

pub type i32Array = Vec<i32>;

/** 'FindBookById' parameters type */
pub struct IFindBookByIdParams {
  commentId: Option<i32>;
}

/** 'FindBookById' return type */
pub struct IFindBookByIdResult {
  id: i32;
  rank: Option<i32>;
  name: Option<String>;
  author_id: Option<i32>;
}

/** 'FindBookById' query type */
pub struct IFindBookByIdQuery {
  params: IFindBookByIdParams;
  result: IFindBookByIdResult;
}

const findBookByIdIR: &str = r#"{"name":"FindBookById","params":[{"name":"commentId","required":false,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":58,"b":66,"line":2,"col":32}]},"isUsed":true}],"statement":{"body":"SELECT * FROM books WHERE id = :commentId","loc":{"a":26,"b":66,"line":2,"col":0}}}"#;

/**
 * Query generated from SQL:
 * ```
 * SELECT * FROM books WHERE id = :commentId
 * ```
 */
export const findBookById = new PreparedQuery<IFindBookByIdParams,IFindBookByIdResult>(findBookByIdIR);


/** 'InsertBooks' parameters type */
pub struct IInsertBooksParams {
  books: Vec<{
    rank: i32,
    name: String,
    authorId: i32
  }>;
}

/** 'InsertBooks' return type */
pub struct IInsertBooksResult {
  book_id: i32;
}

/** 'InsertBooks' query type */
pub struct IInsertBooksQuery {
  params: IInsertBooksParams;
  result: IInsertBooksResult;
}

const insertBooksIR: &str = r#"{"name":"InsertBooks","params":[{"name":"books","codeRefs":{"defined":{"a":106,"b":110,"line":6,"col":9},"used":[{"a":202,"b":206,"line":9,"col":8}]},"transform":{"type":"pick_array_spread","keys":[{"name":"rank","required":true},{"name":"name","required":true},{"name":"authorId","required":true}]},"isUsed":true,"required":false}],"statement":{"body":"INSERT INTO books (rank, name, author_id)\r\nVALUES :books RETURNING id as book_id","loc":{"a":151,"b":230,"line":8,"col":0}}}"#;

/**
 * Query generated from SQL:
 * ```
 * INSERT INTO books (rank, name, author_id)
 * VALUES :books RETURNING id as book_id
 * ```
 */
export const insertBooks = new PreparedQuery<IInsertBooksParams,IInsertBooksResult>(insertBooksIR);


/** 'UpdateBooksCustom' parameters type */
pub struct IUpdateBooksCustomParams {
  rank: Option<i32>;
  id: i32;
}

/** 'UpdateBooksCustom' return type */
export type IUpdateBooksCustomResult = void;

/** 'UpdateBooksCustom' query type */
pub struct IUpdateBooksCustomQuery {
  params: IUpdateBooksCustomParams;
  result: IUpdateBooksCustomResult;
}

const updateBooksCustomIR: &str = r#"{"name":"UpdateBooksCustom","params":[{"name":"rank","required":false,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":324,"b":327,"line":17,"col":20},{"a":371,"b":374,"line":18,"col":23}]},"isUsed":true},{"name":"id","required":true,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":441,"b":443,"line":22,"col":12}]},"isUsed":true}],"statement":{"body":"UPDATE books\r\nSET\r\n    rank = (\r\n        CASE WHEN (:rank::int IS NOT NULL)\r\n                 THEN :rank\r\n             ELSE rank\r\n            END\r\n        )\r\nWHERE id = :id!","loc":{"a":271,"b":443,"line":14,"col":0}}}"#;

/**
 * Query generated from SQL:
 * ```
 * UPDATE books
 * SET
 *     rank = (
 *         CASE WHEN (:rank::int IS NOT NULL)
 *                  THEN :rank
 *              ELSE rank
 *             END
 *         )
 * WHERE id = :id!
 * ```
 */
export const updateBooksCustom = new PreparedQuery<IUpdateBooksCustomParams,IUpdateBooksCustomResult>(updateBooksCustomIR);


/** 'UpdateBooks' parameters type */
pub struct IUpdateBooksParams {
  name: Option<String>;
  rank: Option<i32>;
  id: i32;
}

/** 'UpdateBooks' return type */
export type IUpdateBooksResult = void;

/** 'UpdateBooks' query type */
pub struct IUpdateBooksQuery {
  params: IUpdateBooksParams;
  result: IUpdateBooksResult;
}

const updateBooksIR: &str = r#"{"name":"UpdateBooks","params":[{"name":"name","required":false,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":532,"b":535,"line":30,"col":12}]},"isUsed":true},{"name":"rank","required":false,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":551,"b":554,"line":31,"col":12}]},"isUsed":true},{"name":"id","required":true,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":569,"b":571,"line":32,"col":12}]},"isUsed":true}],"statement":{"body":"UPDATE books\r\n                     \r\nSET\r\n    name = :name,\r\n    rank = :rank\r\nWHERE id = :id!","loc":{"a":478,"b":571,"line":27,"col":0}}}"#;

/**
 * Query generated from SQL:
 * ```
 * UPDATE books
 *                      
 * SET
 *     name = :name,
 *     rank = :rank
 * WHERE id = :id!
 * ```
 */
export const updateBooks = new PreparedQuery<IUpdateBooksParams,IUpdateBooksResult>(updateBooksIR);


/** 'GetBooksByAuthorName' parameters type */
pub struct IGetBooksByAuthorNameParams {
  authorName: String;
}

/** 'GetBooksByAuthorName' return type */
pub struct IGetBooksByAuthorNameResult {
  id: i32;
  rank: Option<i32>;
  name: Option<String>;
  author_id: Option<i32>;
}

/** 'GetBooksByAuthorName' query type */
pub struct IGetBooksByAuthorNameQuery {
  params: IGetBooksByAuthorNameParams;
  result: IGetBooksByAuthorNameResult;
}

const getBooksByAuthorNameIR: &str = r#"{"name":"GetBooksByAuthorName","params":[{"name":"authorName","required":true,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":724,"b":734,"line":37,"col":44}]},"isUsed":true}],"statement":{"body":"SELECT b.* FROM books b\r\nINNER JOIN authors a ON a.id = b.author_id\r\nWHERE a.first_name || ' ' || a.last_name = :authorName!","loc":{"a":611,"b":734,"line":35,"col":0}}}"#;

/**
 * Query generated from SQL:
 * ```
 * SELECT b.* FROM books b
 * INNER JOIN authors a ON a.id = b.author_id
 * WHERE a.first_name || ' ' || a.last_name = :authorName!
 * ```
 */
export const getBooksByAuthorName = new PreparedQuery<IGetBooksByAuthorNameParams,IGetBooksByAuthorNameResult>(getBooksByAuthorNameIR);


/** 'AggregateEmailsAndTest' parameters type */
pub struct IAggregateEmailsAndTestParams {
  testAges: Option<i32Array>;
}

/** 'AggregateEmailsAndTest' return type */
pub struct IAggregateEmailsAndTestResult {
  emails: Option<StringArray>;
  agetest: Option<boolean>;
}

/** 'AggregateEmailsAndTest' query type */
pub struct IAggregateEmailsAndTestQuery {
  params: IAggregateEmailsAndTestParams;
  result: IAggregateEmailsAndTestResult;
}

const aggregateEmailsAndTestIR: &str = r#"{"name":"AggregateEmailsAndTest","params":[{"name":"testAges","required":false,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":829,"b":836,"line":40,"col":53}]},"isUsed":true}],"statement":{"body":"SELECT array_agg(email) as emails, array_agg(age) = :testAges as ageTest FROM users","loc":{"a":776,"b":858,"line":40,"col":0}}}"#;

/**
 * Query generated from SQL:
 * ```
 * SELECT array_agg(email) as emails, array_agg(age) = :testAges as ageTest FROM users
 * ```
 */
export const aggregateEmailsAndTest = new PreparedQuery<IAggregateEmailsAndTestParams,IAggregateEmailsAndTestResult>(aggregateEmailsAndTestIR);


