import copy
import base64
import random
import string
import xml.etree.ElementTree as ET
import json

# # mysql
# import pymysql
# import pymysql.cursors
import pymysql

class Myjson(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, bytes):
            return str(obj, encoding='utf-8')
        return json.JSONEncoder.default(self, obj)

class FPSParser(object):
    def __init__(self, fps_path):
        self.fps_path = fps_path
        self.version = None

    @property
    def _root(self):
        root = ET.ElementTree(file=self.fps_path).getroot()
        self.version = root.attrib.get("version", "No Version")
        if self.version not in ["1.1", "1.2"]:
            raise ValueError("Unsupported version '" + self.version + "'")
        return root

    def parse(self):
        ret = []
        for node in self._root:
            if node.tag == "item":
                ret.append(self._parse_one_problem(node))
        return ret

    def _parse_one_problem(self, node):
        sample_start = True
        test_case_start = True
        problem = {"title": "No Title", "description": "No Description",
                   "input": "No Input Description",
                   "output": "No Output Description",
                   "memory_limit": {"unit": None, "value": None},
                   "time_limit": {"unit": None, "value": None},
                   "samples": [], "images": [], "append": [],
                   "template": [], "prepend": [], "test_cases": [],
                   "hint": None, "source": None, "spj": None, "solution": []}
        for item in node:
            tag = item.tag
            if tag in ["title", "description", "input", "output", "hint", "source"]:
                problem[item.tag] = item.text
            elif tag == "time_limit":
                unit = item.attrib.get("unit", "s")
                if unit not in ["s", "ms"]:
                    raise ValueError("Invalid time limit unit")
                problem["time_limit"]["unit"] = item.attrib.get("unit", "s")
                value = 0
                if self.version != "1.1":
                    value = float(item.text)
                else:
                    value = int(item.text)
                if value <= 0:
                    raise ValueError("Invalid time limit value")
                problem["time_limit"]["value"] = value
            elif tag == "memory_limit":
                unit = item.attrib.get("unit", "MB")
                if unit not in ["MB", "KB", "mb", "kb"]:
                    raise ValueError("Invalid memory limit unit")
                problem["memory_limit"]["unit"] = unit.upper()
                value = int(item.text)
                if value <= 0:
                    raise ValueError("Invalid memory limit value")
                problem["memory_limit"]["value"] = value
            elif tag in ["template", "append", "prepend", "solution"]:
                lang = item.attrib.get("language")
                if not lang:
                    raise ValueError("Invalid " + tag +
                                     ", language name is missed")
                problem[tag].append({"language": lang, "code": item.text})
            elif tag == 'spj':
                lang = item.attrib.get("language")
                if not lang:
                    raise ValueError("Invalid spj, language name if missed")
                problem["spj"] = {"language": lang, "code": item.text}
            elif tag == "img":
                problem["images"].append({"src": None, "blob": None})
                for child in item:
                    if child.tag == "src":
                        problem["images"][-1]["src"] = child.text
                    elif child.tag == "base64":
                        problem["images"][-1]["blob"] = base64.b64decode(
                            child.text)
            elif tag == "sample_input":
                if not sample_start:
                    raise ValueError(
                        "Invalid xml, error 'sample_input' tag order")
                problem["samples"].append({"input": item.text, "output": None})
                sample_start = False
            elif tag == "sample_output":
                if sample_start:
                    raise ValueError(
                        "Invalid xml, error 'sample_output' tag order")
                problem["samples"][-1]["output"] = item.text
                sample_start = True
            elif tag == "test_input":
                if not test_case_start:
                    raise ValueError(
                        "Invalid xml, error 'test_input' tag order")
                problem["test_cases"].append(
                    {"input": item.text, "output": None})
                test_case_start = False
            elif tag == "test_output":
                if test_case_start:
                    raise ValueError(
                        "Invalid xml, error 'test_output' tag order")
                problem["test_cases"][-1]["output"] = item.text
                test_case_start = True

        return problem


class FPSHelper(object):
    def save_image(self, problem, base_dir, base_url):
        pass
        # _problem = copy.deepcopy(problem)
        # for img in _problem["images"]:
        #     name = "".join(random.choice(
        #         string.ascii_lowercase + string.digits) for _ in range(12))
        #     ext = os.path.splitext(img["src"])[1]
        #     file_name = name + ext
        #     with open(os.path.join(base_dir, file_name), "wb") as f:
        #         f.write(img["blob"])
        #     for item in ["description", "input", "output"]:
        #         _problem[item] = _problem[item].replace(
        #             img["src"], os.path.join(base_url, file_name))
        # return _problem

    def save_test_case(self, problem, base_dir, input_preprocessor=None, output_preprocessor=None):
        for index, item in enumerate(problem["test_cases"]):
            with open(os.path.join(base_dir, str(index + 1) + ".in"), "w", encoding="utf-8") as f:
                if input_preprocessor:
                    input_content = input_preprocessor(item["input"])
                else:
                    input_content = item["input"]
                # if input_content is null string, write a newline
                if not input_content:
                    input_content = "\n"
                f.write(input_content)
            with open(os.path.join(base_dir, str(index + 1) + ".out"), "w", encoding="utf-8") as f:
                if output_preprocessor:
                    output_content = output_preprocessor(item["output"])
                else:
                    output_content = item["output"]
                # if output_content is null string, write a newline
                if not output_content:
                    output_content = "\n"
                f.write(output_content)

    def save_problem_info_json(self, problem, base_dir):
        myjson = Myjson()
        # if problem has image 
        # blob->str
        for img in problem["images"]:
            img["blob"] = str(img["blob"])
        with open(os.path.join(base_dir, "problem.json"), "w") as f:
            json.dump(problem, f, indent=4, ensure_ascii=False)
            # myjson.default(problem)
            # f.write(myjson.dumps())



    def save_problem_info_mysql(self, problem, db, index):
        cursor = db.cursor()
        problem_id = index + 1
        problem_title = problem["title"]
        problem_content = problem["description"]
        problem_solution = problem["solution"][0]["code"]
        problem_title = problem_title.replace("\\", "\\\\")
        problem_title = problem_title.replace("'", "\\'")
        print(problem_content)
        problem_content = problem_content.replace("\\", "\\\\")
        print(problem_content)
        problem_content = problem_content.replace("'", "\\'")
        print(problem_content)
        problem_solution = problem_solution.replace("\\", "\\\\")
        problem_solution = problem_solution.replace("'", "\\'")
        sql = "INSERT INTO `tb_problem` (`problem_id`, problem_title, problem_content) VALUES ({0}, '{1}', '{2}')".format(problem_id, problem_title, problem_content)
        cursor.execute(sql)
        cursor.close()
        

def init_db():
    db = pymysql.connect(
        host = "localhost",
        port = 3306,
        user = "RUST-OJ",
        password = "123456",
        db = "rustoj",    
        charset = "utf8mb4"
    )
    return db

if __name__ == "__main__":
    import pprint
    import os

    db = init_db()
    parser = FPSParser("problem_new.xml")
    helper = FPSHelper()
    problems = parser.parse()
    for index, problem in enumerate(problems):
        path = os.path.join("./problems/", str(index + 1))
        os.makedirs(path)
        # helper.save_problem_info_mysql(problem, db, index)
        helper.save_test_case(problem, path)


        helper.save_problem_info_json(problem, path)
        # pprint.pprint(helper.save_image(problem, "/tmp", "/static/img"))

