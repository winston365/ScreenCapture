#include <QApplication>

#include "Lang.h"

Lang* lang;

Lang::Lang(QObject *parent) : QObject(parent)
{
	
}

Lang::~Lang()
{

}

QString Lang::get(const QString& name)
{
	return lang->dic[name];
}

void Lang::init(const QString& langType)
{
	lang = new Lang(qApp);
	if (langType == "zhcn") {
		lang->initZhCn();
	}
	else if (langType == "en") {
		lang->initEn();
	}
	else if (langType == "ko" || langType == "kr") {
		lang->initKo();
	}
}

void Lang::initZhCn()
{
	dic.insert("saveFile", "保存文件");

	dic.insert("rect", "矩形");
	dic.insert("ellipse", "圆形");
	dic.insert("arrow", "箭头");
	dic.insert("number", "标号");
	dic.insert("line", "线条");
	dic.insert("text", "文本");
	dic.insert("mosaic", "马赛克");
	dic.insert("eraser", "橡皮擦");
	dic.insert("undo", "撤销");
	dic.insert("redo", "重做");
	dic.insert("pin", "钉住");
	dic.insert("clipboard", "保存到剪切板");
	dic.insert("save", "保存");
	dic.insert("close", "关闭");

	dic.insert("rectFill", "填充矩形");
	dic.insert("strokeCtrl", "线条粗细：");
	dic.insert("ellipseFill", "填充椭圆");
	dic.insert("arrowFill", "填充箭头");
	dic.insert("numberFill", "填充标号");
	dic.insert("lineTransparent", "半透明线条");
	dic.insert("bold", "粗体");
	dic.insert("italic", "斜体");
	dic.insert("mosaicFill", "矩形马赛克");
	dic.insert("eraserFill", "矩形橡皮擦");

	dic.insert("red", "红");
	dic.insert("yellow", "黄");
	dic.insert("green", "绿");
	dic.insert("cyan", "青");
	dic.insert("blue", "蓝");
	dic.insert("purple", "紫");
	dic.insert("pink", "粉");
	dic.insert("black", "黑");

	dic.insert("start", "开始");
	dic.insert("reachBottom", "已触底，自动滚动停止");
	dic.insert("tooLong", "长图过长，已自动停止");
}

void Lang::initEn()
{
	dic.insert("saveFile", "Save File");
	dic.insert("rect", "Rect");
	dic.insert("ellipse", "Ellipse");
	dic.insert("arrow", "Arrow");
	dic.insert("number", "Number");
	dic.insert("line", "Pen");
	dic.insert("text", "Text");
	dic.insert("mosaic", "Mosaic");
	dic.insert("eraser", "Eraser");
	dic.insert("undo", "Undo");
	dic.insert("redo", "Redo");
	dic.insert("pin", "Pin");
	dic.insert("clipboard", "Save to Clipboard");
	dic.insert("save", "Save to Disk");
	dic.insert("close", "Close");

	dic.insert("rectFill", "Filled Rect");
	dic.insert("strokeCtrl", "Stroke Width: ");
	dic.insert("ellipseFill", "Filled Ellipse");
	dic.insert("arrowFill", "Filled Arrow");
	dic.insert("numberFill", "Filled Number");
	dic.insert("lineTransparent", "Translucent Line");
	dic.insert("bold", "Bold");
	dic.insert("italic", "Italics");
	dic.insert("mosaicFill", "Rectangle Mosaic");
	dic.insert("eraserFill", "Rectangle Eraser");


	dic.insert("red", "Red");
	dic.insert("yellow", "Yellow");
	dic.insert("green", "Green");
	dic.insert("cyan", "Cyan");
	dic.insert("blue", "Blue");
	dic.insert("purple", "Purple");
	dic.insert("pink", "Pink");
	dic.insert("black", "Black");

	dic.insert("start", "Start");
	dic.insert("reachBottom", "Reached the bottom, auto scrolling stopped");
	dic.insert("tooLong", "Image is too long，auto scrolling stopped");
}

void Lang::initKo()
{
	dic.insert("saveFile", "파일 저장");

	dic.insert("rect", "사각형");
	dic.insert("ellipse", "원형");
	dic.insert("arrow", "화살표");
	dic.insert("number", "번호");
	dic.insert("line", "선");
	dic.insert("text", "텍스트");
	dic.insert("mosaic", "모자이크");
	dic.insert("eraser", "지우개");
	dic.insert("undo", "실행 취소");
	dic.insert("redo", "다시 실행");
	dic.insert("pin", "고정");
	dic.insert("clipboard", "클립보드에 저장");
	dic.insert("save", "디스크에 저장");
	dic.insert("close", "닫기");

	dic.insert("rectFill", "채워진 사각형");
	dic.insert("strokeCtrl", "선 굵기: ");
	dic.insert("ellipseFill", "채워진 원형");
	dic.insert("arrowFill", "채워진 화살표");
	dic.insert("numberFill", "채워진 번호");
	dic.insert("lineTransparent", "반투명 선");
	dic.insert("bold", "굵게");
	dic.insert("italic", "기울임");
	dic.insert("mosaicFill", "사각형 모자이크");
	dic.insert("eraserFill", "사각형 지우개");


	dic.insert("red", "빨강");
	dic.insert("yellow", "노랑");
	dic.insert("green", "초록");
	dic.insert("cyan", "청록");
	dic.insert("blue", "파랑");
	dic.insert("purple", "보라");
	dic.insert("pink", "분홍");
	dic.insert("black", "검정");

	dic.insert("start", "시작");
	dic.insert("reachBottom", "하단에 도달하여 자동 스크롤이 중지되었습니다");
	dic.insert("tooLong", "이미지가 너무 길어 자동 스크롤이 중지되었습니다");
}
