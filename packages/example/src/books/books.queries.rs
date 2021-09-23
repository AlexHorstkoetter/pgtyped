/** Types generated for queries found in "src/books/books.sql" */
import { PreparedQuery } from '@pgtyped/query';

export type StringArray = (String)[];

export type i32Array = (i32)[];

/** 'FindBookById' parameters type */
export interface IFindBookByIdParams {
  commentId: i32 | null | void;
}

/** 'FindBookById' return type */
export interface IFindBookByIdResult {
  id: i32;
  rank: i32 | null;
  name: String | null;
  author_id: i32 | null;
}

/** 'FindBookById' query type */
export interface IFindBookByIdQuery {
  params: IFindBookByIdParams;
  result: IFindBookByIdResult;
}

const findBookByIdIR: any = {"name":"FindBookById","params":[{"name":"commentId","required":false,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":58,"b":66,"line":2,"col":32}]}}],"usedParamSet":{"commentId":true},"statement":{"body":"SELECT * FROM books WHERE id = :commentId","loc":{"a":26,"b":66,"line":2,"col":0}}};

/**
 * Query generated from SQL:
 * ```
 * SELECT * FROM books WHERE id = :commentId
 * ```
 */
export const findBookById = new PreparedQuery<IFindBookByIdParams,IFindBookByIdResult>(findBookByIdIR);


/** 'InsertBooks' parameters type */
export interface IInsertBooksParams {
  books: readonly ({
    rank: i32,
    name: String,
    authorId: i32
  })[];
}

/** 'InsertBooks' return type */
export interface IInsertBooksResult {
  book_id: i32;
}

/** 'InsertBooks' query type */
export interface IInsertBooksQuery {
  params: IInsertBooksParams;
  result: IInsertBooksResult;
}

const insertBooksIR: any = {"name":"InsertBooks","params":[{"name":"books","codeRefs":{"defined":{"a":106,"b":110,"line":6,"col":9},"used":[{"a":202,"b":206,"line":9,"col":8}]},"transform":{"type":"pick_array_spread","keys":[{"name":"rank","required":true},{"name":"name","required":true},{"name":"authorId","required":true}]},"required":false}],"usedParamSet":{"books":true},"statement":{"body":"INSERT INTO books (rank, name, author_id)\r\nVALUES :books RETURNING id as book_id","loc":{"a":151,"b":230,"line":8,"col":0}}};

/**
 * Query generated from SQL:
 * ```
 * INSERT INTO books (rank, name, author_id)
 * VALUES :books RETURNING id as book_id
 * ```
 */
export const insertBooks = new PreparedQuery<IInsertBooksParams,IInsertBooksResult>(insertBooksIR);


/** 'UpdateBooksCustom' parameters type */
export interface IUpdateBooksCustomParams {
  rank: i32 | null | void;
  id: i32;
}

/** 'UpdateBooksCustom' return type */
export type IUpdateBooksCustomResult = void;

/** 'UpdateBooksCustom' query type */
export interface IUpdateBooksCustomQuery {
  params: IUpdateBooksCustomParams;
  result: IUpdateBooksCustomResult;
}

const updateBooksCustomIR: any = {"name":"UpdateBooksCustom","params":[{"name":"rank","required":false,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":324,"b":327,"line":17,"col":20},{"a":371,"b":374,"line":18,"col":23}]}},{"name":"id","required":true,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":441,"b":443,"line":22,"col":12}]}}],"usedParamSet":{"rank":true,"id":true},"statement":{"body":"UPDATE books\r\nSET\r\n    rank = (\r\n        CASE WHEN (:rank::int IS NOT NULL)\r\n                 THEN :rank\r\n             ELSE rank\r\n            END\r\n        )\r\nWHERE id = :id!","loc":{"a":271,"b":443,"line":14,"col":0}}};

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
export interface IUpdateBooksParams {
  name: String | null | void;
  rank: i32 | null | void;
  id: i32;
}

/** 'UpdateBooks' return type */
export type IUpdateBooksResult = void;

/** 'UpdateBooks' query type */
export interface IUpdateBooksQuery {
  params: IUpdateBooksParams;
  result: IUpdateBooksResult;
}

const updateBooksIR: any = {"name":"UpdateBooks","params":[{"name":"name","required":false,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":532,"b":535,"line":30,"col":12}]}},{"name":"rank","required":false,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":551,"b":554,"line":31,"col":12}]}},{"name":"id","required":true,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":569,"b":571,"line":32,"col":12}]}}],"usedParamSet":{"name":true,"rank":true,"id":true},"statement":{"body":"UPDATE books\r\n                     \r\nSET\r\n    name = :name,\r\n    rank = :rank\r\nWHERE id = :id!","loc":{"a":478,"b":571,"line":27,"col":0}}};

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
export interface IGetBooksByAuthorNameParams {
  authorName: String;
}

/** 'GetBooksByAuthorName' return type */
export interface IGetBooksByAuthorNameResult {
  id: i32;
  rank: i32 | null;
  name: String | null;
  author_id: i32 | null;
}

/** 'GetBooksByAuthorName' query type */
export interface IGetBooksByAuthorNameQuery {
  params: IGetBooksByAuthorNameParams;
  result: IGetBooksByAuthorNameResult;
}

const getBooksByAuthorNameIR: any = {"name":"GetBooksByAuthorName","params":[{"name":"authorName","required":true,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":724,"b":734,"line":37,"col":44}]}}],"usedParamSet":{"authorName":true},"statement":{"body":"SELECT b.* FROM books b\r\nINNER JOIN authors a ON a.id = b.author_id\r\nWHERE a.first_name || ' ' || a.last_name = :authorName!","loc":{"a":611,"b":734,"line":35,"col":0}}};

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
export interface IAggregateEmailsAndTestParams {
  testAges: i32Array | null | void;
}

/** 'AggregateEmailsAndTest' return type */
export interface IAggregateEmailsAndTestResult {
  emails: StringArray | null;
  agetest: boolean | null;
}

/** 'AggregateEmailsAndTest' query type */
export interface IAggregateEmailsAndTestQuery {
  params: IAggregateEmailsAndTestParams;
  result: IAggregateEmailsAndTestResult;
}

const aggregateEmailsAndTestIR: any = {"name":"AggregateEmailsAndTest","params":[{"name":"testAges","required":false,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":829,"b":836,"line":40,"col":53}]}}],"usedParamSet":{"testAges":true},"statement":{"body":"SELECT array_agg(email) as emails, array_agg(age) = :testAges as ageTest FROM users","loc":{"a":776,"b":858,"line":40,"col":0}}};

/**
 * Query generated from SQL:
 * ```
 * SELECT array_agg(email) as emails, array_agg(age) = :testAges as ageTest FROM users
 * ```
 */
export const aggregateEmailsAndTest = new PreparedQuery<IAggregateEmailsAndTestParams,IAggregateEmailsAndTestResult>(aggregateEmailsAndTestIR);


